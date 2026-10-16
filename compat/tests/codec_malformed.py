"""New codec truncation/corruption rejection and public C mutation boundaries."""
import os,pathlib,struct,subprocess,tempfile,unittest
from coverage import assemble,OUT,ROOT
from lerc_compat import scene
from lerc_reference import encode
from webp_compat import encode_scene,tiff

class CodecMalformed(unittest.TestCase):
 def test_lerc_corruption(self):
  w,h=7,5;blob=encode(bytes((i*17)%256 for i in range(w*h)),1,w,h)
  def wrap(data):
   tags=[(256,4,[w]),(257,4,[h]),(258,3,[8]),(259,3,[34887]),(262,3,[1]),(277,3,[1]),(50674,4,[4,0]),(273,4,('SEG',[data])),(279,4,[len(data)]),(278,4,[h])]
   return assemble([tags],False,True)[0]
  with tempfile.TemporaryDirectory() as td:
   path=pathlib.Path(td);src=path/'in.tif';dst=path/'out'
   cases=[blob[:n] for n in range(len(blob))]+[blob+b'\0']
   for at in range(10,len(blob)):
    v=bytearray(blob);v[at]^=1;cases.append(bytes(v))
   for data in cases:
    src.write_bytes(wrap(data));r=subprocess.run([str(OUT/'dump'),str(src),'native',str(dst),'0','1'],capture_output=True)
    self.assertNotEqual(r.returncode,0)
   print(f'LERC malformed: {len(cases)} truncations/checksum/ending failures rejected')
 def test_new_codec_mutations(self):
  with tempfile.TemporaryDirectory() as td:
   path=pathlib.Path(td);seeds=[]
   for kind,wrapper in ((1,0),(6,1),(7,2)):
    data,_=scene(kind,wrapper=wrapper);seed=path/f'lerc-{kind}-{wrapper}.tif';seed.write_bytes(data);seeds.append(str(seed))
   for tiled,lossless,alpha in ((False,True,False),(True,True,True),(False,False,False),(True,False,True)):
    data,_=tiff(encode_scene(path,tiled,lossless,alpha),4,tiled=tiled);seed=path/f'webp-{tiled}-{lossless}-{alpha}.tif';seed.write_bytes(data);seeds.append(str(seed))
   r=subprocess.run([str(OUT/'mutate-test'),*seeds],capture_output=True,env=dict(os.environ,VTIFF_FUZZ_WORK='100000000'),timeout=180)
   self.assertEqual(r.returncode,0,r.stderr);print(r.stdout.decode().strip())
if __name__=='__main__':unittest.main(verbosity=2)
