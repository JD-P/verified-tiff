"""Native ZSTD TIFF integration, independent samples and a pinned enabled oracle."""
import itertools,json,os,pathlib,subprocess,tempfile,unittest
from coverage import ROOT,OUT,scene,assemble
from zstd_reference import compress
class ZstdCompat(unittest.TestCase):
 def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
 def tearDown(self):self.tmp.cleanup()
 def dump(self,blob,mode,expected=None,oracle=True,success=True):
  source=self.path/'source.tif';source.write_bytes(blob);ours=self.path/'ours';ref=self.path/'reference'
  r=subprocess.run([str(OUT/'dump'),str(source),mode,str(ours),'0','1'],capture_output=True,timeout=30)
  if not success:self.assertNotEqual(r.returncode,0);return
  self.assertEqual(r.returncode,0,r.stderr);data=ours.read_bytes()
  if expected is not None:self.assertEqual(data,expected)
  if oracle:
   r=subprocess.run([str(OUT/'dump-libtiff-zstd'),str(source),mode,str(ref),'0','1'],capture_output=True,timeout=30)
   self.assertEqual(r.returncode,0,r.stderr);self.assertEqual(data,ref.read_bytes())
  return data
 def test_native_layouts_predictors_fill_orders_and_bigtiff(self):
  cases=refs=0
  profiles=[(1,1,1,1,0),(4,1,1,1,0),(8,1,3,2,0),(16,1,4,2,2),(32,2,1,2,0),(64,1,3,2,0),(32,3,3,3,0),(64,3,3,3,0)]
  for bits,fmt,ch,pred,alpha in profiles:
   for le,big,tiled,separate,fill in itertools.product([False,True],[False,True],[False,True],[False,True],[1,2]):
    blob,native,scan,*_=scene(w=17,h=19,bits=bits,fmt=fmt,ch=ch,predict=pred,alpha=alpha,codec=50000,le=le,big=big,tiled=tiled,separate=separate,fill=fill)
    oracle=not(bits==64 and pred==2)
    with self.subTest(bits=bits,fmt=fmt,ch=ch,pred=pred,le=le,big=big,tiled=tiled,separate=separate,fill=fill):
     self.dump(blob,'native',native,oracle)
     if not tiled:self.dump(blob,'scan',scan,oracle)
    cases+=1;refs+=oracle
  (OUT/'zstd-compat.json').write_text(json.dumps({'independent_native_files':cases,'pinned_libtiff_files':refs,'libtiff':'4.7.1','zstd':'1.5.7'},indent=2)+'\n');print('ZSTD TIFF:',cases,'independent files,',refs,'pinned-oracle files')
 def test_rgba_color_and_alpha(self):
  for photo,ch,bits,alpha,tiled in [(1,1,8,0,False),(2,3,8,0,True),(2,4,8,2,False),(2,4,16,1,True),(5,4,8,0,False),(8,3,16,0,True)]:
   self.dump(scene(w=17,h=19,photo=photo,ch=ch,bits=bits,alpha=alpha,tiled=tiled,codec=50000,predict=2)[0],'rgba')
 def test_temporary_user_buffer_contexts(self):
  for bits,fmt,pred,le,big,tiled in itertools.product([32,64],[3],[3],[False,True],[False,True],[False,True]):
   blob=scene(w=17,h=19,bits=bits,fmt=fmt,ch=3,predict=pred,codec=50000,le=le,big=big,tiled=tiled)[0]
   p=self.path/'clone.tif';p.write_bytes(blob)
   r=subprocess.run([str(OUT/'consumer-test'),'clone',str(p)],capture_output=True,timeout=30);self.assertEqual(r.returncode,0,r.stderr)
 def test_bounded_final_strip_padding_and_checksum_rejection(self):
  _,native,_,tags,*_=scene(w=7,h=5,ch=1,codec=1)
  source=next(v[1] for k,_,v in tags if k==273)
  for extra,success in [(7,True),(8,False)]:
   segments=[compress(source[0]),compress(source[1]+bytes(extra))]
   t=[(k,ty,[50000] if k==259 else ('SEG',segments) if k==273 else list(map(len,segments)) if k==279 else v) for k,ty,v in tags]
   self.dump(assemble([t])[0],'native',native,oracle=False,success=success)
  segments=[compress(source[0]),bytearray(compress(source[1]))];segments[1][-1]^=1;segments=list(map(bytes,segments))
  t=[(k,ty,[50000] if k==259 else ('SEG',segments) if k==273 else list(map(len,segments)) if k==279 else v) for k,ty,v in tags]
  self.dump(assemble([t])[0],'native',success=False)
if __name__=='__main__':unittest.main(verbosity=2)
