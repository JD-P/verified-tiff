import itertools,json,os,pathlib,struct,subprocess,tempfile,unittest
from fixtures import make_tiff,entry,change_inline,lzw_codes,lzw_encode

ROOT=pathlib.Path(__file__).resolve().parents[1]
BIN=ROOT/'target/debug/verified-tiff'
ORACLE=ROOT/'target/libtiff-oracle'


class Integration(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
    def tearDown(self): self.tmp.cleanup()
    def run_file(self,data,*flags):
        src=self.path/'input.tif';out=self.path/'output.pnm'
        src.write_bytes(data)
        p=subprocess.run([str(BIN),str(src),str(out),*map(str,flags)],capture_output=True,timeout=5)
        return p,out
    def test_profile_matrix(self):
        cases=0
        for le,tiled,before,channels,white in itertools.product([False,True],[False,True],[False,True],[1,3],[False,True]):
            if channels==3 and white: continue
            for w,h in [(1,1),(7,5),(17,19),(32,16)]:
                with self.subTest(le=le,tiled=tiled,before=before,channels=channels,white=white,w=w,h=h):
                    data,pixels=make_tiff(w,h,channels,white,le,tiled,before)
                    p,out=self.run_file(data)
                    self.assertEqual(p.returncode,0,p.stderr)
                    meta=json.loads(p.stdout);self.assertEqual(meta['output_bytes'],len(pixels[0]))
                    self.assertEqual(out.read_bytes().split(b'\n',3)[3],pixels[0])
                    if ORACLE.exists():
                        ref=self.path/'oracle.ppm'
                        q=subprocess.run([str(ORACLE),str(self.path/'input.tif'),str(ref),'0'],capture_output=True,timeout=5)
                        self.assertEqual(q.returncode,0,q.stderr)
                        expected=pixels[0] if channels==3 else bytes(c for v in pixels[0] for c in (v,v,v))
                        self.assertEqual(ref.read_bytes().split(b'\n',3)[3],expected)
                    cases+=1
        print(f'profile matrix: {cases} cases; oracle={ORACLE.exists()}')
    def test_pages(self):
        data,pixels=make_tiff(pages=3)
        for page in range(3):
            p,out=self.run_file(data,'--page',page)
            self.assertEqual(p.returncode,0,p.stderr)
            self.assertEqual(out.read_bytes().split(b'\n',3)[3],pixels[page])
        self.assertIn(b'PageNotFound',self.run_file(data,'--page',3)[0].stderr)
    def check_bilevel(self,data,expected,page=0,**limits):
        flags=['--page',page]
        for name,value in limits.items(): flags.extend(['--'+name.replace('_','-'),value])
        p,out=self.run_file(data,*flags)
        self.assertEqual(p.returncode,0,p.stderr)
        meta=json.loads(p.stdout)
        self.assertEqual(meta['channels'],1)
        self.assertEqual(meta['bits_per_sample'],1)
        self.assertEqual(meta['output_bytes'],len(expected))
        self.assertEqual(out.read_bytes().split(b'\n',3)[3],expected)
        self.assertTrue(out.read_bytes().startswith(b'P5\n'))
        if ORACLE.exists():
            ref=self.path/'oracle.ppm'
            q=subprocess.run([str(ORACLE),str(self.path/'input.tif'),str(ref),str(page)],capture_output=True,timeout=5)
            self.assertEqual(q.returncode,0,q.stderr)
            self.assertEqual(ref.read_bytes().split(b'\n',3)[3],bytes(c for v in expected for c in (v,v,v)))
        return meta
    def test_bilevel_matrix(self):
        cases=0
        for le,tiled,before,white,fill in itertools.product([False,True],[False,True],[False,True],[False,True],[1,2]):
            for w,h in [(1,1),(7,5),(8,7),(9,19),(15,3),(16,16),(17,19),(31,5),(32,16),(33,17)]:
                with self.subTest(le=le,tiled=tiled,before=before,white=white,fill=fill,w=w,h=h):
                    data,pixels=make_tiff(w,h,white=white,le=le,tiled=tiled,before=before,bits=1,fill=fill)
                    self.check_bilevel(data,pixels[0]);cases+=1
        print(f'bilevel matrix: {cases} cases; oracle={ORACLE.exists()}')
    def test_bilevel_all_byte_patterns(self):
        # Each row represents a different byte; check every bit position/value.
        for le,tiled,white,fill in itertools.product([False,True],[False,True],[False,True],[1,2]):
            data,pixels=make_tiff(8,256,le=le,tiled=tiled,white=white,bits=1,fill=fill,
                pixel_value=lambda x,y,c,p: 255 if y & (1 << (7-x)) else 0)
            self.check_bilevel(data,pixels[0])
    def test_bilevel_defaults_and_pages(self):
        for le,tiled,white,fill in itertools.product([False,True],[False,True],[False,True],[1,2]):
            data,pixels=make_tiff(9,5,le=le,tiled=tiled,white=white,bits=1,fill=fill,rows=5,pages=3,
                extra={258:None,259:None,277:None,278:None})
            for page in range(3): self.check_bilevel(data,pixels[page],page)
    def test_bilevel_limits(self):
        data,pixels=make_tiff(33,17,bits=1)
        used=self.check_bilevel(data,pixels[0])['work_used']
        # Limits and decode work count expanded output bytes, not packed storage.
        self.check_bilevel(data,pixels[0],work=used,max_output=33*17,max_input=len(data))
        for option,value in [('work',used-1),('max-output',33*17-1),('max-output',5*17)]:
            p,_=self.run_file(data,'--'+option,value)
            self.assertEqual(p.returncode,2);self.assertIn(b'Limit',p.stderr)
    def test_bilevel_invalid_and_truncated(self):
        data,_=make_tiff(9,3,bits=1,rows=3)
        for n in range(len(data)):
            p,_=self.run_file(data[:n]);self.assertEqual(p.returncode,2,(n,p.stderr))
        # Two bytes per row are required even though nine bits would fit in a
        # continuous 27-bit stream. Declared/truncated storage must be rejected.
        for count in [0,3,4,5]:
            p,_=self.run_file(change_inline(data,279,count))
            self.assertEqual(p.returncode,2);self.assertIn(b'Truncated',p.stderr)
        tile,_=make_tiff(1,1,bits=1,tiled=True)
        p,_=self.run_file(change_inline(tile,325,31))
        # Even a one-pixel image needs the full 16x16 padded tile (32 bytes).
        self.assertEqual(p.returncode,2);self.assertIn(b'Truncated',p.stderr)
        for depth in [0,2,4,32]:
            p,_=self.run_file(change_inline(data,258,depth))
            self.assertEqual(p.returncode,2);self.assertIn(b'Unsupported',p.stderr)
        invalid=[make_tiff(bits=1,extra={266:(3,[3])})[0],
                 make_tiff(extra={266:(3,[2])})[0],
                 make_tiff(channels=3,extra={258:None})[0],
                 make_tiff(channels=3,extra={258:(3,[1,1,1])})[0],
                 make_tiff(channels=3,extra={258:(3,[8,1,8])})[0],
                 make_tiff(bits=1,extra={258:(4,[1])})[0],
                 make_tiff(bits=1,extra={258:(3,[1,1])})[0],
                 make_tiff(bits=1,extra={258:(3,[])})[0]]
        for bad in invalid:
            p,_=self.run_file(bad);self.assertEqual(p.returncode,2,p.stderr)
    def test_defaults(self):
        data,pixels=make_tiff(rows=5,extra={259:None,277:None,278:None})
        p,out=self.run_file(data);self.assertEqual(p.returncode,0,p.stderr)
        self.assertEqual(out.read_bytes().split(b'\n',3)[3],pixels[0])
    def check_compressed(self,data,expected,channels=1,bits=8,page=0,compression=32773):
        p,out=self.run_file(data,'--page',page)
        self.assertEqual(p.returncode,0,p.stderr)
        meta=json.loads(p.stdout)
        self.assertEqual(meta['compression'],compression)
        self.assertEqual(meta['bits_per_sample'],bits)
        self.assertEqual(meta['output_bytes'],len(expected))
        self.assertEqual(out.read_bytes().split(b'\n',3)[3],expected)
        if ORACLE.exists():
            ref=self.path/'oracle.ppm'
            q=subprocess.run([str(ORACLE),str(self.path/'input.tif'),str(ref),str(page)],capture_output=True,timeout=5)
            self.assertEqual(q.returncode,0,q.stderr)
            rgb=expected if channels==3 else bytes(c for v in expected for c in (v,v,v))
            self.assertEqual(ref.read_bytes().split(b'\n',3)[3],rgb)
        return meta
    def test_packbits_matrix(self):
        cases=0
        formats=[(1,1,False),(1,1,True),(8,1,False),(8,1,True),(8,3,False)]
        for (bits,channels,white),le,tiled,before,literal in itertools.product(formats,[False,True],[False,True],[False,True],[False,True]):
            for w,h in [(1,1),(7,5),(9,19),(17,19),(32,16),(129,3),(257,5),(1025,2)]:
                with self.subTest(bits=bits,channels=channels,white=white,le=le,tiled=tiled,before=before,literal=literal,w=w,h=h):
                    data,pixels=make_tiff(w,h,channels=channels,white=white,le=le,tiled=tiled,before=before,bits=bits,
                        compression=32773,literal_only=literal,noops=literal)
                    self.check_compressed(data,pixels[0],channels,bits);cases+=1
        print(f'PackBits matrix: {cases} cases; oracle={ORACLE.exists()}')
    def test_packbits_repeats_and_controls(self):
        for bits,channels in [(1,1),(8,1),(8,3)]:
            for width in [1,2,3,127,128,129,256,257,1025]:
                data,pixels=make_tiff(width,5,bits=bits,channels=channels,compression=32773,noops=True,
                    pixel_value=lambda x,y,c,p: 255 if y%2 else 0)
                self.check_compressed(data,pixels[0],channels,bits)
        for control in range(256):
            if control==128:
                encoded=b'\x80\x00\x2a';pixels=b'\x2a'
            elif control<128:
                pixels=bytes(range(control+1));encoded=bytes([control])+pixels
            else:
                pixels=b'\x2a'*(257-control);encoded=bytes([control,42])
            data,_=make_tiff(len(pixels),1,compression=32773,rows=1,encoded_block=lambda _,e=encoded:e)
            self.check_compressed(data,pixels)
    def test_packbits_defaults_and_pages(self):
        for bits,channels in [(1,1),(8,1),(8,3)]:
            data,pixels=make_tiff(17,19,bits=bits,channels=channels,tiled=True,le=False,before=True,pages=3,
                compression=32773,extra={258:None} if bits==1 else None)
            for page in range(3): self.check_compressed(data,pixels[page],channels,bits,page)
    def test_packbits_malformed(self):
        bad_packets=[b'',b'\x80',b'\x00',b'\xff',b'\x7f\x00',b'\x81\x00',
                     b'\xff\x2a',b'\x00\x2a\x00\x77',b'\x00\x2a\x7f']
        for bits in [1,8]:
            for packet in bad_packets:
                data,_=make_tiff(1,1,bits=bits,compression=32773,encoded_block=lambda _,p=packet:p)
                p,_=self.run_file(data);self.assertEqual(p.returncode,2,(bits,packet,p.stderr))
            # Exact total output length is insufficient if a packet crosses rows.
            data,_=make_tiff(1,2,rows=2,bits=bits,compression=32773,encoded_block=lambda _:b'\x01\x12\x34')
            p,_=self.run_file(data);self.assertEqual(p.returncode,2);self.assertIn(b'Invalid',p.stderr)
            data,_=make_tiff(9,3,bits=bits,compression=32773)
            for n in range(len(data)):
                p,_=self.run_file(data[:n]);self.assertEqual(p.returncode,2,(bits,n,p.stderr))
        data,_=make_tiff(bits=1,fill=2,compression=32773)
        self.assertIn(b'Unsupported',self.run_file(data)[0].stderr)
    def test_packbits_work_and_output_limits(self):
        for bits in [1,8]:
            data,pixels=make_tiff(257,5,bits=bits,tiled=True,compression=32773,pixel_value=lambda x,y,c,p:255)
            meta=self.check_compressed(data,pixels[0],bits=bits)
            p,_=self.run_file(data,'--work',meta['work_used'],'--max-output',len(pixels[0]))
            self.assertEqual(p.returncode,0,p.stderr)
            for flag,value in [('work',meta['work_used']-1),('max-output',len(pixels[0])-1)]:
                p,_=self.run_file(data,'--'+flag,value);self.assertEqual(p.returncode,2);self.assertIn(b'Limit',p.stderr)
            source=self.path/'input.tif'
            q=subprocess.run([str(BIN),str(source),'--inspect'],capture_output=True,timeout=5)
            self.assertEqual(q.returncode,0,q.stderr)
            inspected=json.loads(q.stdout)
            self.assertEqual(meta['work_used']-inspected['work_used'],meta['decode_work'])
    def test_lzw_matrix(self):
        cases=0
        formats=[(1,1,False),(1,1,True),(8,1,False),(8,1,True),(8,3,False)]
        for (bits,channels,white),le,tiled,before in itertools.product(formats,[False,True],[False,True],[False,True]):
            for w,h in [(1,1),(7,5),(9,19),(17,19),(32,16),(129,3),(257,5),(1025,2)]:
                with self.subTest(bits=bits,channels=channels,white=white,le=le,tiled=tiled,before=before,w=w,h=h):
                    data,pixels=make_tiff(w,h,channels=channels,white=white,le=le,tiled=tiled,before=before,bits=bits,compression=5)
                    self.check_compressed(data,pixels[0],channels,bits,compression=5);cases+=1
        print(f'LZW matrix: {cases} cases; oracle={ORACLE.exists()}')
    def test_lzw_code_boundaries_and_resets(self):
        # Literal-only streams isolate every early-width transition, including
        # when the very next code is EOI or Clear. The expected bytes are direct.
        for n in [1,2,252,253,254,255,764,765,766,767,768,1788,1789,1790,1791,1792,3837,3838,3839]:
            for reset in [False,True]:
                raw=bytes(i%256 for i in range(n))
                codes=[256,*raw]
                if reset: codes.extend([256,91]);raw+=b'['
                codes.append(257)
                data,_=make_tiff(len(raw),1,rows=1,compression=5,encoded_block=lambda _,c=codes:lzw_codes(c))
                self.check_compressed(data,raw,compression=5)
        # Dictionary self-reference (KwKwK), existing words, and fresh tables.
        for codes,raw in [([256,65,258,259,260,257],b'A'*10),
                          ([256,65,66,258,260,257],b'ABABABA'),
                          ([256,256,65,258,256,66,258,257],b'AAABBB'),
                          ([256,65,256,256,257],b'A')]:
            data,_=make_tiff(len(raw),1,rows=1,compression=5,encoded_block=lambda _,c=codes:lzw_codes(c))
            self.check_compressed(data,raw,compression=5)
        # The largest representable phrase: 3,838 new entries plus literals fit
        # the fixed table; the final KwKwK entry has length 3,839 (code 4095).
        codes=[256,65,*range(258,4096),257]
        length=3839*3840//2
        data,_=make_tiff(1920,3839,rows=3839,compression=5,pixel_value=lambda *args:65,
                         encoded_block=lambda _:lzw_codes(codes))
        self.check_compressed(data,b'A'*length,compression=5)
    def test_lzw_random_and_libtiff_encoder(self):
        import random
        rng=random.Random(0x1a2b3c)
        for bits,channels in [(1,1),(8,1),(8,3)]:
            width,height=257,131
            raw=[rng.randrange(2)*255 if bits==1 else rng.randrange(256)
                 for _ in range(width*height*channels)]
            data,pixels=make_tiff(width,height,channels=channels,bits=bits,rows=height,compression=5,
                pixel_value=lambda x,y,c,p:raw[(y*width+x)*channels+c])
            self.check_compressed(data,pixels[0],channels,bits,compression=5)
        # Committed upstream LZW fixtures use libtiff's encoder. This test also
        # exercises a second encoder on deterministic noisy, multi-reset data.
        tiffcp=ROOT/'.tools/tiff-4.7.1/tools/tiffcp'
        if tiffcp.exists():
            for channels in [1,3]:
                raw=bytes(rng.randrange(256) for _ in range(257*131*channels))
                data,pixels=make_tiff(257,131,channels=channels,rows=131,
                    pixel_value=lambda x,y,c,p:raw[(y*257+x)*channels+c])
                src=self.path/'plain.tif';dest=self.path/'encoded.tif';src.write_bytes(data)
                q=subprocess.run([str(tiffcp),'-c','lzw:1','-r','131',str(src),str(dest)],capture_output=True,timeout=10)
                self.assertEqual(q.returncode,0,q.stderr)
                self.check_compressed(dest.read_bytes(),pixels[0],channels,compression=5)
    def test_lzw_defaults_pages_and_limits(self):
        for bits,channels in [(1,1),(8,1),(8,3)]:
            data,pixels=make_tiff(17,19,bits=bits,channels=channels,tiled=True,le=False,before=True,pages=3,
                compression=5,extra={258:None} if bits==1 else None)
            for page in range(3): self.check_compressed(data,pixels[page],channels,bits,page,compression=5)
            meta=self.check_compressed(data,pixels[0],channels,bits,compression=5)
            p,_=self.run_file(data,'--work',meta['work_used'],'--max-output',len(pixels[0]))
            self.assertEqual(p.returncode,0,p.stderr)
            for flag,value in [('work',meta['work_used']-1),('max-output',len(pixels[0])-1)]:
                p,_=self.run_file(data,'--'+flag,value);self.assertEqual(p.returncode,2);self.assertIn(b'Limit',p.stderr)
            q=subprocess.run([str(BIN),str(self.path/'input.tif'),'--inspect'],capture_output=True,timeout=5)
            self.assertEqual(q.returncode,0,q.stderr)
            self.assertEqual(meta['work_used']-json.loads(q.stdout)['work_used'],meta['decode_work'])
    def test_lzw_malformed_and_truncated(self):
        valid=lzw_codes([256,42,257])
        # A sub-byte truncated EOI after exact output is covered in lzw_tail.py.
        bad=[b'',b'\x80',valid+b'\0',valid+b'\xff',
             lzw_codes([42,257]),lzw_codes([256,257]),lzw_codes([256,258,257]),
             lzw_codes([256,42,259,257]),lzw_codes([256,42,258,257]),
             lzw_codes([256,42,42,257]),lzw_codes([256,42,256,258,257])]
        for bits in [1,8]:
            for encoded in bad:
                data,_=make_tiff(1,1,bits=bits,compression=5,encoded_block=lambda _,e=encoded:e)
                p,_=self.run_file(data);self.assertEqual(p.returncode,2,(bits,encoded,p.stderr))
            data,_=make_tiff(9,3,bits=bits,compression=5)
            for n in range(len(data)):
                p,_=self.run_file(data[:n]);self.assertEqual(p.returncode,2,(bits,n,p.stderr))
        # GIF/PDF-style late width changes are not TIFF LZW.
        raw=bytes(i%256 for i in range(800))
        data,_=make_tiff(800,1,rows=1,compression=5,encoded_block=lambda _:lzw_codes([256,*raw,257],late_change=True))
        self.assertEqual(self.run_file(data)[0].returncode,2)
        # Data after a full dictionary must be preceded by Clear.
        codes=[256,*([42]*3840),257]
        data,_=make_tiff(3840,1,compression=5,encoded_block=lambda _:lzw_codes(codes))
        self.assertEqual(self.run_file(data)[0].returncode,2)
        # Unused low bits of the last byte are padding and need not be zero.
        data,_=make_tiff(1,1,compression=5,encoded_block=lambda _:valid[:-1]+bytes([valid[-1]|31]))
        self.check_compressed(data,b'*',compression=5)
        for data in [make_tiff(bits=1,fill=2,compression=5)[0],
                     make_tiff(compression=5,extra={317:(3,[2])})[0]]:
            self.assertIn(b'Unsupported',self.run_file(data)[0].stderr)
    def test_truncation(self):
        data,_=make_tiff(channels=3)
        for n in range(len(data)):
            p,_=self.run_file(data[:n]);self.assertEqual(p.returncode,2,(n,p.stderr))
    def test_malformed(self):
        data,_=make_tiff()
        mutations=[change_inline(data,256,0),change_inline(data,257,0),change_inline(data,278,0),
                   change_inline(data,273,0xffffffff),change_inline(data,256,0xffffffff)]
        d=bytearray(data);struct.pack_into('<I',d,entry(d,258)+4,0xffffffff);mutations.append(bytes(d))
        d=bytearray(data);struct.pack_into('<H',d,entry(d,257),256);mutations.append(bytes(d))
        d=bytearray(data);struct.pack_into('<H',d,entry(d,258)+2,65535);mutations.append(bytes(d))
        for d in mutations:
            p,_=self.run_file(d);self.assertEqual(p.returncode,2,p.stderr)
    def test_unsupported(self):
        data,_=make_tiff()
        for d in [change_inline(data,259,99),change_inline(data,258,32),change_inline(data,262,5),
                  make_tiff(extra={274:(3,[6])})[0],make_tiff(extra={284:(3,[3])})[0],
                  make_tiff(extra={338:(3,[1])})[0],make_tiff(extra={339:(3,[3])})[0]]:
            p,_=self.run_file(d);self.assertIn(b'Unsupported',p.stderr)
        d=bytearray(data);struct.pack_into('<H',d,2,43)
        self.assertIn(b'Unsupported',self.run_file(d)[0].stderr)
    def test_limits(self):
        data,_=make_tiff();p,_=self.run_file(data);used=json.loads(p.stdout)['work_used']
        for option,value in [('work',used-1),('max-input',len(data)-1),('max-output',34),
                             ('max-dimension',6),('max-directories',0),('max-tags',1),('max-segments',1)]:
            with self.subTest(option=option):
                p,_=self.run_file(data,'--'+option,value);self.assertIn(b'Limit',p.stderr)
        p,_=self.run_file(data,'--work',used,'--max-output',35,'--max-input',len(data))
        self.assertEqual(p.returncode,0,p.stderr)
    def test_cycle_bounded(self):
        data,_=make_tiff();d=bytearray(data);off=struct.unpack_from('<I',d,4)[0];n=struct.unpack_from('<H',d,off)[0]
        struct.pack_into('<I',d,off+2+12*n,off)
        p,_=self.run_file(d,'--page',1000000);self.assertIn(b'Limit',p.stderr)
        p,_=self.run_file(d,'--page',100,'--work',50);self.assertIn(b'Limit',p.stderr)

if __name__=='__main__':unittest.main(verbosity=2)
