"""Complete lossy VP8 YUV comparisons with pinned libwebp."""
import pathlib,random,subprocess,tempfile,unittest
ROOT=pathlib.Path(__file__).resolve().parents[1]
ENC=ROOT/'target/webp-encode';DEC=ROOT/'target/vp8-yuv-oracle';EXE=ROOT/'target/debug/examples/vp8-native'
class NativeVP8(unittest.TestCase):
 def test_matrix(self):
  rng=random.Random(831012);cases=0
  with tempfile.TemporaryDirectory() as td:
   td=pathlib.Path(td);raw=td/'rgba';encoded=td/'image.webp';decoded=td/'yuv'
   for w,h in ((1,1),(1,33),(33,1),(17,19),(65,67),(129,127),(256,257)):
    for pattern in ('constant','ramp','checker','random','regions'):
     palette=[bytes([rng.randrange(256),rng.randrange(256),rng.randrange(256),255]) for _ in range(256)];pixels=[]
     for y in range(h):
      for x in range(w):
       if pattern=='constant':p=bytes([37,51,199,255])
       elif pattern=='ramp':p=bytes([(x*3+y*7)%256,(x+y*2)%256,(x*17+y)%256,255])
       elif pattern=='checker':p=palette[(x//3+y//2)%16]
       elif pattern=='regions':p=bytes([x%256,y%256,(x+y)%256,255]) if x<w//2 else palette[(x//7+y//5)%16]
       else:p=bytes([rng.randrange(256),rng.randrange(256),rng.randrange(256),255])
       pixels.append(p)
     raw.write_bytes(b''.join(pixels))
     for method,quality in ((0,0),(1,35),(4,75),(6,100)):
      e=subprocess.run([str(ENC),str(raw),str(w),str(h),'0',str(quality),str(method),'0',str(encoded)],capture_output=True);self.assertEqual(e.returncode,0,e.stderr)
      ref=subprocess.run([str(DEC),str(encoded),str(decoded)],capture_output=True);self.assertEqual(ref.returncode,0,ref.stderr)
      run=subprocess.run([str(EXE),str(encoded),str(w),str(h)],capture_output=True);expected=decoded.read_bytes()
      if run.returncode or run.stdout!=expected:
       (ROOT/'target/vp8-failure.webp').write_bytes(encoded.read_bytes());(ROOT/'target/vp8-failure.expected').write_bytes(expected);(ROOT/'target/vp8-failure.actual').write_bytes(run.stdout)
      self.assertEqual(run.returncode,0,(w,h,pattern,method,quality,run.stderr))
      self.assertEqual(run.stdout,expected,(w,h,pattern,method,quality))
      ref=subprocess.run([str(ROOT/'target/webp-oracle'),str(encoded),str(decoded)],capture_output=True);self.assertEqual(ref.returncode,0,ref.stderr)
      run=subprocess.run([str(EXE),str(encoded),str(w),str(h),'rgba'],capture_output=True);expected=decoded.read_bytes()
      if run.returncode or run.stdout!=expected:
       (ROOT/'target/vp8-rgb-failure.webp').write_bytes(encoded.read_bytes());(ROOT/'target/vp8-rgb-failure.expected').write_bytes(expected);(ROOT/'target/vp8-rgb-failure.actual').write_bytes(run.stdout)
      self.assertEqual(run.returncode,0,(w,h,pattern,method,quality,run.stderr));self.assertEqual(run.stdout,expected,(w,h,pattern,method,quality));cases+=1
  print(f'Lossy VP8: {cases} complete images matched exact libwebp YUV and RGBA')
if __name__=='__main__':unittest.main(verbosity=2)
