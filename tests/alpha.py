"""Alpha integration: independent native-sample oracle and rational reference."""
import itertools,json,pathlib,subprocess,tempfile,unittest
from fixtures import make_tiff,canonical_alpha,lzw_codes

ROOT=pathlib.Path(__file__).resolve().parents[1]
BIN=ROOT/'target/debug/verified-tiff'
ORACLE=ROOT/'target/alpha-oracle'

class Alpha(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
    def tearDown(self): self.tmp.cleanup()
    def run_file(self,data,*flags):
        src=self.path/'input.tif';out=self.path/'output.pam';src.write_bytes(data)
        p=subprocess.run([str(BIN),str(src),str(out),*map(str,flags)],capture_output=True,timeout=10)
        return p,out
    def check(self,data,expected,channels,white,mode,page=0,oracle=True):
        p,out=self.run_file(data,'--page',page)
        self.assertEqual(p.returncode,0,p.stderr)
        meta=json.loads(p.stdout)
        self.assertEqual(meta['channels'],channels)
        self.assertEqual(meta['bits_per_sample'],8)
        self.assertEqual(meta['source_alpha'],'associated' if mode==1 else 'unassociated')
        self.assertEqual(meta['output_alpha'],'straight')
        self.assertEqual(meta['output_bytes'],len(expected))
        header,raw=out.read_bytes().split(b'ENDHDR\n',1)
        tuple_name='GRAYSCALE_ALPHA' if channels==2 else 'RGB_ALPHA'
        self.assertEqual(header,f"P7\nWIDTH {meta['width']}\nHEIGHT {meta['height']}\nDEPTH {channels}\nMAXVAL 255\nTUPLTYPE {tuple_name}\n".encode())
        self.assertEqual(raw,expected)
        if oracle and ORACLE.exists():
            ref=self.path/'oracle.raw'
            q=subprocess.run([str(ORACLE),str(self.path/'input.tif'),str(ref),str(page)],capture_output=True,timeout=10)
            self.assertEqual(q.returncode,0,q.stderr)
            native=ref.read_bytes()
            self.assertEqual(canonical_alpha(native,channels,white,mode),expected)
            self.assertEqual(raw[channels-1::channels],native[channels-1::channels])
        return meta
    def test_alpha_matrix(self):
        cases=0
        for (ch,white),mode,codec,le,tiled,before in itertools.product(
                [(2,False),(2,True),(4,False)],[1,2],[1,32773,5],[False,True],[False,True],[False,True]):
            for w,h in [(1,1),(7,5),(17,19),(32,16)]:
                with self.subTest(ch=ch,white=white,mode=mode,codec=codec,le=le,tiled=tiled,before=before,w=w,h=h):
                    data,pixels=make_tiff(w,h,channels=ch,white=white,alpha=mode,compression=codec,le=le,tiled=tiled,before=before)
                    self.check(data,pixels[0],ch,white,mode);cases+=1
        print(f'alpha matrix: {cases} cases; native libtiff oracle={ORACLE.exists()}')
    def test_all_color_alpha_pairs(self):
        # Every possible stored component and opacity, including color > alpha.
        cases=0
        for (ch,white),mode in itertools.product([(2,False),(2,True),(4,False)],[1,2]):
            data,pixels=make_tiff(256,256,channels=ch,white=white,alpha=mode,rows=256,
                stored_value=lambda x,y,c,p: y if c==ch-1 else (x+c*37)%256)
            self.check(data,pixels[0],ch,white,mode);cases+=1
        print(f'alpha exhaustive conversion: {cases} images, 65536 color/alpha pairs per color channel')
    def test_explicit_alpha_semantics(self):
        cases=[(4,False,2,[7,89,255,0],[7,89,255,0]),
               (4,False,1,[7,89,255,0],[0,0,0,0]),
               (4,False,1,[32,64,200,128],[64,128,255,128]),
               (2,False,1,[1,2],[128,2]),
               (2,True,1,[32,128],[191,128]),
               (2,True,1,[255,0],[0,0]),
               (2,True,2,[32,128],[223,128])]
        for ch,white,mode,native,expected in cases:
            for codec in [1,32773,5]:
                data,_=make_tiff(1,1,channels=ch,white=white,alpha=mode,compression=codec,
                    stored_value=lambda x,y,c,p:native[c])
                self.check(data,bytes(expected),ch,white,mode)
    def test_pages_formats_and_limits(self):
        for ch,mode,codec in itertools.product([2,4],[1,2],[1,32773,5]):
            data,pixels=make_tiff(17,19,channels=ch,alpha=mode,compression=codec,tiled=True,le=False,before=True,pages=3,
                extra={339:(3,[1]*ch)})
            for page in range(3): self.check(data,pixels[page],ch,False,mode,page)
            meta=self.check(data,pixels[0],ch,False,mode)
            q=subprocess.run([str(BIN),str(self.path/'input.tif'),'--inspect'],capture_output=True,timeout=5)
            self.assertEqual(q.returncode,0,q.stderr)
            self.assertEqual(meta['work_used']-json.loads(q.stdout)['work_used'],meta['decode_work'])
            self.assertGreaterEqual(meta['decode_work'],2*len(pixels[0]))
            if codec==1:self.assertEqual(meta['decode_work'],2*len(pixels[0]))
            p,_=self.run_file(data,'--work',meta['work_used'],'--max-output',len(pixels[0]),'--max-input',len(data))
            self.assertEqual(p.returncode,0,p.stderr)
            for flag,value in [('work',meta['work_used']-1),('max-output',len(pixels[0])-1)]:
                (self.path/'output.pam').write_bytes(b'keep previous output')
                p,out=self.run_file(data,'--'+flag,value)
                self.assertEqual(p.returncode,2);self.assertIn(b'Limit',p.stderr)
                self.assertEqual(out.read_bytes(),b'keep previous output')
    def test_invalid_alpha_metadata(self):
        tags=[{338:None},{338:(3,[])},{338:(3,[1,2])},{338:(4,[1])},
              {338:(3,[0])},{338:(3,[3])},{338:(3,[65535])},
              {258:None},{258:(3,[1,1,1,1])},{258:(3,[8,8,8,16])},
              {258:(3,[8,8,8])},{258:(3,[16]*4)},
              {277:(3,[3])},{262:(3,[1])},{266:(3,[2])},{284:(3,[2])},
              {317:(3,[2])},{339:(3,[1,1])},{339:(3,[1,1,1,3])}]
        for codec in [1,32773,5]:
            for extra in tags:
                data,_=make_tiff(channels=4,alpha=2,compression=codec,extra=extra)
                p,_=self.run_file(data);self.assertEqual(p.returncode,2,(extra,p.stderr))
            for ch in [1,3]:
                for mode in [1,2]:
                    data,_=make_tiff(channels=ch,compression=codec,extra={338:(3,[mode])})
                    self.assertEqual(self.run_file(data)[0].returncode,2)
            data,_=make_tiff(channels=2,alpha=1,compression=codec,extra={262:(3,[2])})
            self.assertEqual(self.run_file(data)[0].returncode,2)
    def test_truncated_alpha_storage(self):
        for codec in [1,32773,5]:
            # Put samples after all metadata: every proper prefix loses required data.
            data,_=make_tiff(3,2,channels=2,alpha=1,compression=codec,rows=2,extra={65000:None})
            for n in range(len(data)):
                p,_=self.run_file(data[:n]);self.assertEqual(p.returncode,2,(codec,n,p.stderr))
        for codec,encoded in [(32773,b'\x02\x01\x02\x03'),(5,lzw_codes([256,1,2,3,257]))]:
            # Three colors without the required fourth alpha byte are incomplete.
            data,_=make_tiff(1,1,channels=4,alpha=2,compression=codec,encoded_block=lambda _,e=encoded:e)
            self.assertEqual(self.run_file(data)[0].returncode,2)
    def test_independent_libtiff_encoder(self):
        tiffcp=ROOT/'.tools/tiff-4.7.1/tools/tiffcp'
        if not tiffcp.exists():self.skipTest('libtiff encoder not built')
        import random
        rng=random.Random(93812)
        for ch,mode in itertools.product([2,4],[1,2]):
            native=bytes(rng.randrange(256) for _ in range(257*131*ch))
            data,pixels=make_tiff(257,131,channels=ch,alpha=mode,rows=131,
                stored_value=lambda x,y,c,p:native[(y*257+x)*ch+c])
            src=self.path/'plain.tif';dest=self.path/'encoded.tif';src.write_bytes(data)
            for codec in ['packbits','lzw:1']:
                q=subprocess.run([str(tiffcp),'-c',codec,'-r','131',str(src),str(dest)],capture_output=True,timeout=10)
                self.assertEqual(q.returncode,0,q.stderr)
                self.check(dest.read_bytes(),pixels[0],ch,False,mode)

if __name__=='__main__':unittest.main(verbosity=2)
