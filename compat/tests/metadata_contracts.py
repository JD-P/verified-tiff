"""Codec ABI and ink consistency, including an independent libtiff 4.7.1 oracle."""
import itertools,pathlib,struct,subprocess,tempfile,unittest
from coverage import assemble,OUT

# tag, storage kind, values, compression; constants from TIFF and upstream ABI.
CODECS=[(292,4,[0],3),(293,4,[0],4),(326,4,[17],3),(327,3,[1],3),
 (328,4,[29],3),(317,3,[2],5),(347,7,b'\xff\xd8\xff\xd9',7),
 (512,3,[1],6),(513,4,[17],6),(514,4,[29],6),(515,3,[7],6),
 (519,4,[17,29],6),(520,4,[17,29],6),(521,4,[17,29],6),(50674,4,[4,0],34887),
 (34908,4,[17],1),(34909,2,b'subaddress\0',1),(34910,4,[29],1),(34911,2,b'dcs\0',1)]
def base(codec=1):
 return [(256,4,[1]),(257,4,[1]),(258,3,[1 if codec in (3,4) else 8]),
  (259,3,[codec]),(262,3,[1]),(273,4,('SEG',[b'%'])),(277,3,[1]),(278,4,[1]),(279,4,[1])]
def run(mode,f,*args,oracle=False):
 cmd=[str(OUT/('metadata-contracts-libtiff' if oracle else 'metadata-contracts')),mode,str(f),*map(str,args)]
 p=subprocess.run(cmd,capture_output=True)
 assert p.returncode==0,(cmd,p.returncode,p.stdout,p.stderr)
 return p.stdout
class MetadataContracts(unittest.TestCase):
 def test_codec_inventory_and_typed_getters(self):
  with tempfile.TemporaryDirectory() as d:
   f=pathlib.Path(d)/'codec.tif'
   for le,big,(tag,kind,values,codec) in itertools.product([False,True],[False,True],CODECS):
    f.write_bytes(assemble([base(codec)+[(tag,kind,values)]],le=le,big=big)[0])
    got=run('codec',f,tag,1);want=run('codec',f,tag,1,oracle=True)
    self.assertEqual(got,want,(le,big,tag))
 def test_known_tags_never_use_anonymous_argument_shape(self):
  with tempfile.TemporaryDirectory() as d:
   f=pathlib.Path(d)/'bad.tif'
   for le,big,tag in itertools.product([False,True],[False,True],[326,327,328,512,513,514,515,34908,34910]):
    for kind,values in [(12,struct.pack(('<' if le else '>')+'d',1.0)),(3,[1,2])]:
     f.write_bytes(assemble([base()+[(tag,kind,values)]],le=le,big=big)[0]);run('codec',f,tag,0)
   # The name and ABI also remain built in when the codec does not own the image.
   for tag,kind,values,_ in CODECS:
    f.write_bytes(assemble([base()+[(tag,kind,values)]])[0]);run('codec',f,tag,1)
    f.write_bytes(assemble([base()])[0]);run('codec',f,tag,0)
   for tag,kind,values in [(326,16,[1<<32]),(328,16,[1<<32]),(327,4,[65536]),(512,4,[65536]),(515,4,[65536])]:
    f.write_bytes(assemble([base()+[(tag,kind,values)]])[0]);run('codec',f,tag,0)
 def test_ink_normalization_matches_reference(self):
  with tempfile.TemporaryDirectory() as d:
   f=pathlib.Path(d)/'ink.tif'
   for le,big,names,declared in itertools.product([False,True],[False,True],
       [b'cyan\0magenta\0',b'\0',b'\0\0x\0',b'a'*65534+b'\0',b'\0'*65535],[None,0,1,9,65535]):
    tags=base()+[(333,2,names)]+([] if declared is None else [(334,3,[declared])])
    f.write_bytes(assemble([tags],le=le,big=big)[0]);n=names.count(0)
    self.assertEqual(run('ink',f,n,len(names)),str(n).encode()+b'\n'+names)
    self.assertEqual(run('ink',f,n,len(names),oracle=True),str(n).encode()+b'\n'+names)
 def test_invalid_ink_payloads_are_rejected(self):
  with tempfile.TemporaryDirectory() as d:
   f=pathlib.Path(d)/'badink.tif'
   for le,big,(kind,names) in itertools.product([False,True],[False,True],
       [(2,b''),(2,b'cyan'),(2,b'cyan\0magenta'),(2,b'\0'*65536),(1,b'cyan\0')]):
    f.write_bytes(assemble([base()+[(333,kind,names),(334,3,[9])]],le=le,big=big)[0]);run('reject',f)
   for values in [[],[1,2],[65536]]:
    f.write_bytes(assemble([base()+[(334,4,values)]])[0]);run('reject',f)
 def test_setters_are_atomic_and_keep_names_consistent(self):
  with tempfile.TemporaryDirectory() as d:
   f=pathlib.Path(d)/'set.tif'
   for le,big in itertools.product([False,True],[False,True]):
    f.write_bytes(assemble([base()],le=le,big=big)[0]);run('mutate',f);run('mutate',f,oracle=True)
   for mode in ['wl+','wb+','wl8+','wb8+']:
    run('writer',f,mode);run('ink',f,2,13)
 def test_directory_failure_preserves_metadata_and_namespace(self):
  with tempfile.TemporaryDirectory() as d:
   f=pathlib.Path(d)/'pages.tif'
   for le,big in itertools.product([False,True],[False,True]):
    pages=[base(),base()+[(333,2,b'cyan\0magenta\0'),(334,3,[9])],base()+[(333,2,b'no-nul')]]
    f.write_bytes(assemble(pages,main_next=[1,2,None],le=le,big=big)[0]);run('select',f);run('namespace',f)
if __name__=='__main__':unittest.main()
