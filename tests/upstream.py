#!/usr/bin/env python3
import json,pathlib,subprocess,tempfile,unittest
from fixtures import canonical_alpha
from extended import pixels
from jpeg_fixtures import oracle as jpeg_oracle
ROOT=pathlib.Path(__file__).resolve().parents[1]
BIN=ROOT/'target/debug/verified-tiff';ORACLE=ROOT/'target/libtiff-oracle'

class UpstreamFixtures(unittest.TestCase):
    def test_adopted_images(self):
        manifest=json.loads((ROOT/'tests/upstream/manifest.json').read_text())
        with tempfile.TemporaryDirectory() as temp:
            temp=pathlib.Path(temp)
            for case in manifest['fixtures']:
                with self.subTest(file=case['file']):
                    source=ROOT/'tests/upstream'/case['file'];out=temp/'ours.pnm'
                    p=subprocess.run([str(BIN),str(source),str(out)],capture_output=True,timeout=5)
                    self.assertEqual(p.returncode,0,p.stderr)
                    metadata=json.loads(p.stdout)
                    self.assertEqual([metadata['width'],metadata['height'],metadata['channels']],case['shape'])
                    alpha=case.get('alpha',0)
                    raw=pixels(out)
                    import hashlib
                    self.assertEqual(hashlib.sha256(raw).hexdigest(),case['pixel_sha256'])
                    if case.get("jpeg_oracle"):
                        expected=jpeg_oracle(source,temp/"reference.raw")
                        self.assertEqual(hashlib.sha256(expected).hexdigest(),case["reference_pixel_sha256"])
                        self.assertEqual(len(raw),len(expected))
                        self.assertLessEqual(max(abs(a-b) for a,b in zip(raw,expected)),case["max_channel_error"])
                    elif ORACLE.exists():
                        ref=temp/'reference.ppm'
                        oracle=ROOT/'target/native-oracle' if case.get('native_oracle') else ROOT/'target/alpha-oracle' if alpha else ORACLE
                        q=subprocess.run([str(oracle),str(source),str(ref),'0'],capture_output=True,timeout=5)
                        self.assertEqual(q.returncode,0,q.stderr)
                        if case.get("native_oracle"):
                            expected=ref.read_bytes()
                            if alpha: expected=canonical_alpha(expected,metadata["channels"],case["white"],alpha)
                            elif case["white"]: expected=bytes(255-v for v in expected)
                            self.assertEqual(raw,expected)
                        elif alpha:
                            self.assertEqual(raw,canonical_alpha(ref.read_bytes(),metadata['channels'],case['white'],alpha))
                        else:
                            rgb=raw if metadata['channels']==3 else bytes(c for v in raw for c in (v,v,v))
                            self.assertEqual(rgb,ref.read_bytes().split(b'\n',3)[3])
            print(f"upstream fixtures: {len(manifest['fixtures'])}; live oracle={ORACLE.exists()}")
    def test_cyclic_requests_are_bounded(self):
        for name in ['test_ifd_loop_to_self.tif','test_ifd_loop_to_first.tif']:
            p=subprocess.run([str(BIN),str(ROOT/'tests/upstream'/name),'--inspect','--page','1024'],capture_output=True,timeout=5)
            self.assertEqual(p.returncode,2);self.assertIn(b'Limit',p.stderr)
            p=subprocess.run([str(BIN),str(ROOT/'tests/upstream'/name),'--inspect','--page','100','--work','10'],capture_output=True,timeout=5)
            self.assertEqual(p.returncode,2);self.assertIn(b'Limit',p.stderr)

if __name__=='__main__':unittest.main(verbosity=2)
