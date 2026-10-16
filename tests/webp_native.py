"""General VP8L streams encoded and decoded by pinned libwebp."""
import pathlib,random,subprocess,tempfile,unittest
ROOT=pathlib.Path(__file__).resolve().parents[1]
EXE=ROOT/'target/debug/examples/webp-native'
ENC=ROOT/'target/webp-encode';DEC=ROOT/'target/webp-oracle'

class NativeWebP(unittest.TestCase):
    def test_lossless_matrix(self):
        rng=random.Random(831011);cases=0
        with tempfile.TemporaryDirectory() as td:
            td=pathlib.Path(td);raw=td/'rgba';encoded=td/'image.webp';decoded=td/'decoded'
            for w,h in ((1,1),(1,33),(33,1),(17,19),(65,67),(129,127)):
                for pattern in ('constant','ramp','palette2','palette4','palette16','palette256','random','regions'):
                    palette=[bytes(rng.randrange(256) for _ in range(4)) for _ in range(256)]
                    pixels=[]
                    for y in range(h):
                        for x in range(w):
                            if pattern=='constant':p=bytes([37,51,199,129])
                            elif pattern=='ramp':p=bytes([(x*3+y*7)%256,(x+y*2)%256,(x*17+y)%256,(x*9+y*13)%256])
                            elif pattern.startswith('palette'):p=palette[(x//3+y//2)%int(pattern[7:])]
                            elif pattern=='regions':p=bytes([x%256,y%256,(x+y)%256,255]) if x<w//2 else palette[(x//7+y//5)%16]
                            else:p=bytes(rng.randrange(256) for _ in range(4))
                            pixels.append(p)
                    expected=b''.join(pixels);raw.write_bytes(expected)
                    for method,quality,hint in ((0,0,0),(1,35,1),(4,75,2),(6,100,3)):
                        enc=subprocess.run([str(ENC),str(raw),str(w),str(h),'1',str(quality),str(method),str(hint),str(encoded)],capture_output=True)
                        self.assertEqual(enc.returncode,0,enc.stderr)
                        ref=subprocess.run([str(DEC),str(encoded),str(decoded)],capture_output=True)
                        self.assertEqual(ref.returncode,0,ref.stderr);self.assertEqual(decoded.read_bytes(),expected)
                        run=subprocess.run([str(EXE),str(encoded),str(w),str(h)],capture_output=True)
                        if run.returncode or run.stdout!=expected:
                            dest=ROOT/'target/webp-failure.webp';dest.write_bytes(encoded.read_bytes())
                        self.assertEqual(run.returncode,0,(w,h,pattern,method,run.stderr))
                        self.assertEqual(run.stdout,expected,(w,h,pattern,method));cases+=1
        print(f'General VP8L: {cases} encoder/reference cases matched exact RGBA pixels')

if __name__=='__main__':unittest.main(verbosity=2)
