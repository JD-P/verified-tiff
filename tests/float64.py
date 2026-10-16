"""Bitwise float64 references; never compare decoded samples using float equality."""
import ast,itertools,json,pathlib,struct,subprocess,tempfile,unittest
from extended import make_extended,pixels,ROOT,BIN,ORACLE
from fixtures import change_inline,entry

PATTERNS=[0,0x8000000000000000,0x3ff0000000000000,0xbff0000000000000,
          0x7ff0000000000000,0xfff0000000000000,0x7ff8000000001234,
          0xfff0000000000001,1,0x8000000000000001,0x0010000000000000,
          0x7fefffffffffffff,0xffffffffffffffff,0x123456789abcdef0]
CODECS=[(1,1),(32773,1),(5,1),(8,1),(32946,1),(5,2),(8,2),(32946,2)]

def fixture(**kw):
    return make_extended(bits=64,sample_values=PATTERNS,**kw)

def npy(path):
    b=path.read_bytes();assert b[:8]==b'\x93NUMPY\x01\x00'
    length=int.from_bytes(b[8:10],'little');assert (10+length)%64==0
    header=b[10:10+length];assert header.endswith(b'\n')
    d=ast.literal_eval(header.decode('ascii'))
    assert d['descr']=='<f8' and d['fortran_order'] is False
    return d,pixels(path)

class Float64(unittest.TestCase):
    def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
    def tearDown(self):self.tmp.cleanup()
    def run_data(self,data,*flags):
        src=self.path/'in.tif';out=self.path/'out.npy';src.write_bytes(data)
        p=subprocess.run([str(BIN),str(src),str(out),*map(str,flags)],capture_output=True,timeout=15)
        return p,out
    def check(self,options,page=0,pages=1):
        data,expected,_=fixture(**options,pages=pages)
        p,out=self.run_data(data,'--page',page);self.assertEqual(p.returncode,0,p.stderr)
        header,actual=npy(out);self.assertEqual(actual,expected[page],options)
        meta=json.loads(p.stdout);ch=options.get('ch',3)
        self.assertEqual(header['shape'],(options.get('h',19),options.get('w',17))+(() if ch==1 else (ch,)))
        self.assertEqual(meta['sample_format'],'ieee_float');self.assertEqual(meta['output_byte_order'],'little')
        self.assertEqual(meta['bits_per_sample'],64);self.assertEqual(meta['output_bits_per_sample'],64)
        self.assertEqual(meta['predictor'],options.get('predictor',1));self.assertEqual(meta['scratch_bytes'],len(actual))
        if ORACLE.exists():
            ref=self.path/'native.raw'
            q=subprocess.run([str(ORACLE),str(self.path/'in.tif'),str(ref),str(page)],capture_output=True,timeout=15)
            self.assertEqual(q.returncode,0,q.stderr);self.assertEqual(ref.read_bytes(),actual,options)
        return data,meta
    def test_format_matrix(self):
        n=0
        for ch,le,tiled,separate,(compression,predictor),(w,h) in itertools.product([1,3],[False,True],[False,True],[False,True],CODECS,[(1,1),(17,19)]):
            with self.subTest(ch=ch,le=le,tiled=tiled,separate=separate,compression=compression,predictor=predictor,w=w,h=h):
                self.check(dict(w=w,h=h,ch=ch,le=le,tiled=tiled,separate=separate,compression=compression,predictor=predictor));n+=1
        print(f'float64 matrix: {n}; bit-exact native oracle={ORACLE.exists()}')
    def test_rows_tiles_pages_and_limits(self):
        n=0
        for le,(compression,predictor) in itertools.product([False,True],[(5,2),(8,2)]):
            options=dict(w=33,h=17,ch=3,le=le,tiled=True,separate=True,compression=compression,predictor=predictor,before=True)
            for page in range(3):self.check(options,page=page,pages=3);n+=1
            data,meta=self.check(options);n+=1
            p,_=self.run_data(data,'--work',meta['work_used']);self.assertEqual(p.returncode,0,p.stderr)
            for flag,value in [('--work',meta['work_used']-1),('--max-output',meta['output_bytes']-1)]:
                p,_=self.run_data(data,flag,value);self.assertEqual(p.returncode,2)
        print(f'float64 page/edge cases: {n}')
    def test_invalid_metadata_and_truncation(self):
        for compression,predictor in [(1,1),(5,2),(8,2)]:
            data,_,_=fixture(w=2,h=1,ch=1,compression=compression,predictor=predictor)
            for end in range(len(data)):
                p,_=self.run_data(data[:end]);self.assertEqual(p.returncode,2,(compression,end))
            for tag,value in [(258,32),(339,1),(339,2),(339,4),(317,3),(262,0),(266,2),(277,2)]:
                p,_=self.run_data(change_inline(data,tag,value));self.assertEqual(p.returncode,2,(compression,tag,value))
            bad=bytearray(data);pos=entry(bad,339);bad[pos:pos+2]=struct.pack('<H',340)
            p,_=self.run_data(bad);self.assertEqual(p.returncode,2)
        for compression in [1,32773]:
            data,_,_=fixture(w=2,h=1,ch=1,compression=compression,predictor=2)
            p,_=self.run_data(data);self.assertEqual(p.returncode,2)
        data,_,_=fixture(w=2,h=1,ch=3,compression=5,predictor=2)
        for tag,value in [(258,32),(339,1)]:
            bad=bytearray(data);p=entry(bad,tag);at=struct.unpack_from('<I',bad,p+8)[0]
            struct.pack_into('<H',bad,at+2,value)
            result,_=self.run_data(bad);self.assertEqual(result.returncode,2)
    def test_original_float64_regressions(self):
        decoded=[]
        for endian in ['be','le']:
            src=ROOT/f'.tools/tiff-4.7.1/test/images/test_float64_predictor2_{endian}_lzw.tif'
            if not src.exists():self.skipTest('upstream originals unavailable')
            p,out=self.run_data(src.read_bytes());self.assertEqual(p.returncode,0,p.stderr)
            header,actual=npy(out);self.assertEqual(header['shape'],(20,20))
            ref=self.path/'native.raw';q=subprocess.run([str(ORACLE),str(src),str(ref),'0'],capture_output=True)
            self.assertEqual(q.returncode,0,q.stderr);self.assertEqual(ref.read_bytes(),actual);decoded.append(actual)
        self.assertEqual(decoded[0],decoded[1])

if __name__=='__main__':unittest.main(verbosity=2)
