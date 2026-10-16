"""Full static VP8/VP8L TIFF paths against independent libwebp and libtiff."""
import itertools,pathlib,struct,subprocess,tempfile,unittest
from coverage import assemble,OUT,ROOT
ENC=ROOT/'target/webp-encode';DEC=ROOT/'target/webp-oracle'
def chunks(blob):
 p=12;out=[]
 while p<len(blob):
  n=struct.unpack_from('<I',blob,p+4)[0];out.append((blob[p:p+4],blob[p+8:p+8+n]));p+=8+n+(n&1)
 return out
def riff(parts):
 data=b'WEBP'+b''.join(tag+struct.pack('<I',len(v))+v+bytes(len(v)&1) for tag,v in parts)
 return b'RIFF'+struct.pack('<I',len(data))+data
def filtered(alpha,w,mode):
 out=bytearray()
 for i,v in enumerate(alpha):
  if mode==0 or i==0:p=0
  elif i<w:p=alpha[i-1]
  elif i%w==0:p=alpha[i-w]
  elif mode==1:p=alpha[i-1]
  elif mode==2:p=alpha[i-w]
  else:p=max(0,min(255,alpha[i-1]+alpha[i-w]-alpha[i-w-1]))
  out.append((v-p)%256)
 return bytes(out)
def encode_scene(path,tiled,lossless,alpha,options=(),raw_filter=None):
 w,h=35,33;bw,bh=(16,16) if tiled else (w,11);segments=[];decoded=[]
 raw=path/'raw';webp=path/'webp';ref=path/'reference'
 for by in range((h+bh-1)//bh):
  for bx in range((w+bw-1)//bw):
   rows=bh if tiled else min(bh,h-by*bh);pixels=bytearray()
   for y in range(rows):
    for x in range(bw):
     xx=x+bx*bw;yy=y+by*bh
     pixels.extend(((xx*13+yy*7)%256,(xx*3+yy*5)%256,(xx*23+yy*11)%256,(xx*19+yy*31)%256 if alpha else 255))
   raw.write_bytes(pixels)
   cmd=[str(ENC),str(raw),str(bw),str(rows),str(int(lossless)),'73','6','0',str(webp)]+list(map(str,options))
   run=subprocess.run(cmd,capture_output=True);assert run.returncode==0,run.stderr
   blob=webp.read_bytes()
   if raw_filter is not None:
    blob=riff([(tag,bytes([raw_filter<<2])+filtered(pixels[3::4],bw,raw_filter) if tag==b'ALPH' else data) for tag,data in chunks(blob)]);webp.write_bytes(blob)
   run=subprocess.run([str(DEC),str(webp),str(ref)],capture_output=True);assert run.returncode==0,run.stderr
   segments.append(blob);decoded.append(ref.read_bytes())
 return w,h,bw,bh,segments,decoded

def tiff(scene,ch=4,big=False,le=True,tiled=False):
 w,h,bw,bh,segments,decoded=scene;ot=16 if big else 4
 tags=[(256,4,[w]),(257,4,[h]),(258,3,[8]*ch),(259,3,[50001]),(262,3,[2]),(277,3,[ch]),(284,3,[1]),
       (324 if tiled else 273,ot,('SEG',segments)),(325 if tiled else 279,ot,list(map(len,segments)))]
 tags.extend([(322,4,[bw]),(323,4,[bh])] if tiled else [(278,4,[bh])])
 if ch==4:tags.append((338,3,[2]))
 native=b''.join(bytes(v for i,v in enumerate(s) if i%4<ch) for s in decoded)
 return assemble([tags],big,le)[0],native

class WebPCompatibility(unittest.TestCase):
 def check(self,path,scene,ch=4,big=False,le=True,tiled=False,oracle=True):
  blob,expected=tiff(scene,ch,big,le,tiled);source=path/'in.tif';out=path/'out';ref=path/'ref';source.write_bytes(blob)
  run=subprocess.run([str(OUT/'dump'),str(source),'native',str(out),'0','1'],capture_output=True)
  if run.returncode or out.read_bytes()!=expected:(ROOT/'target/webp-tiff-failure.tif').write_bytes(blob)
  self.assertEqual(run.returncode,0,run.stderr);self.assertEqual(out.read_bytes(),expected)
  if oracle:
   run=subprocess.run([str(OUT/'dump-libtiff'),str(source),'native',str(ref),'0','1'],capture_output=True);self.assertEqual(run.returncode,0,run.stderr);self.assertEqual(out.read_bytes(),ref.read_bytes())
   run=subprocess.run([str(OUT/'dump'),str(source),'rgba',str(out),'0','1'],capture_output=True);self.assertEqual(run.returncode,0,run.stderr)
   run=subprocess.run([str(OUT/'dump-libtiff'),str(source),'rgba',str(ref),'0','1'],capture_output=True);self.assertEqual(run.returncode,0,run.stderr);self.assertEqual(out.read_bytes(),ref.read_bytes())
  return blob
 def test_tiff_matrix(self):
  with tempfile.TemporaryDirectory() as td:
   path=pathlib.Path(td);cases=0
   for tiled,lossless,alpha in itertools.product((False,True),repeat=3):
    scene=encode_scene(path,tiled,lossless,alpha)
    for ch,big,le in itertools.product((3,4),(False,True),(False,True)):
     with self.subTest(tiled=tiled,lossless=lossless,alpha=alpha,ch=ch,big=big,le=le):self.check(path,scene,ch,big,le,tiled,oracle=not(ch==3 and alpha))
     cases+=1
   print(f'WebP C API: {cases} layouts matched libwebp; 48 also matched libtiff (RGB-with-alpha extension checked independently)')
 def test_alpha_filters_and_vp8_options(self):
  with tempfile.TemporaryDirectory() as td:
   path=pathlib.Path(td);cases=0
   for compression,filtering,quality,parts,filter_type,strength,segments in [(a,f,q,p,t,s,g) for a in (0,1) for f,q,p,t,s,g in ((0,100,0,0,0,1),(1,70,1,0,30,2),(2,100,2,1,60,3),(2,35,3,1,100,4))]:
    scene=encode_scene(path,False,False,True,(compression,filtering,quality,parts,filter_type,strength,segments));self.check(path,scene);cases+=1
   for mode in range(4):
    scene=encode_scene(path,True,False,True,(0,0,100,0,1,60,4),mode);self.check(path,scene,tiled=True);cases+=1
   print(f'WebP alpha/filter/partition configurations: {cases}')
if __name__=='__main__':unittest.main(verbosity=2)
