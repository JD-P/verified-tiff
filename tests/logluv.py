"""SGILog native LogLuv components, file rejection and resource regressions."""
import ast,hashlib,itertools,json,pathlib,struct,subprocess,tempfile,unittest
from extended import ROOT,BIN,ORACLE,pixels
from logluv_fixtures import make,native,encode
from fixtures import change_inline


class LogluvTests(unittest.TestCase):
    def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
    def tearDown(self):self.tmp.cleanup()
    def run_data(self,data,*flags):
        src=self.path/'input.tif';out=self.path/'output.npy';src.write_bytes(data)
        p=subprocess.run([str(BIN),str(src),str(out),*map(str,flags)],capture_output=True,timeout=30)
        return p,out
    def check(self,data,expected,w,h,page=0,oracle=True):
        p,out=self.run_data(data,'--page',page);self.assertEqual(p.returncode,0,p.stderr)
        meta=json.loads(p.stdout);self.assertEqual(pixels(out),expected)
        self.assertEqual((meta['sample_format'],meta['color_model'],meta['output_byte_order']),('signed_integer','logluv16','little'))
        self.assertEqual((meta['bits_per_sample'],meta['output_bits_per_sample'],meta['channels'],meta['compression']),(16,16,3,34676))
        b=out.read_bytes();self.assertEqual(b[:8],b'\x93NUMPY\x01\x00')
        n=int.from_bytes(b[8:10],'little');header=ast.literal_eval(b[10:10+n].decode())
        self.assertEqual(header,dict(descr='<i2',fortran_order=False,shape=(h,w,3)))
        if oracle:
            ref=self.path/'reference.raw';q=subprocess.run([str(ORACLE),str(self.path/'input.tif'),str(ref),str(page)],capture_output=True,timeout=30)
            self.assertEqual(q.returncode,0,q.stderr);self.assertEqual(ref.read_bytes(),expected)
        return meta
    def test_format_matrix(self):
        cases=0
        for (w,h),le,tiled,mode,pattern in itertools.product([(1,1),(17,19),(128,2),(129,3)],[False,True],[False,True],['mixed','literal','noops'],['varied','runs']):
            with self.subTest(w=w,h=h,le=le,tiled=tiled,mode=mode,pattern=pattern):
                data,expected,_=make(w,h,le,tiled,mode=mode,pattern=pattern)
                self.check(data,expected[0],w,h);cases+=1
        self.assertEqual(cases,96)
        print(f'LogLuv format matrix: {cases}; exact rational reference and libtiff native components')
    def test_all_chroma_values_and_luminance_signs(self):
        words=[(l<<16)|(u<<8)|(255-u) for l in [0,1,0x7fff,0x8000,0x8001,0xffff] for u in range(256)]
        for le in [False,True]:
            data,expected,_=make(256,6,le,rows=2,words=words)
            self.check(data,expected[0],256,6)
        self.assertEqual(len(native(words)),9216)
    def test_packet_boundaries_and_pages(self):
        for w,mode in itertools.product([1,2,126,127,128,129,130,257],['mixed','literal','noops']):
            data,expected,_=make(w,3,rows=2,mode=mode,pattern='runs')
            self.check(data,expected[0],w,3)
        for le,tiled,page in itertools.product([False,True],[False,True],[0,1]):
            data,expected,_=make(17,19,le,tiled,pages=2)
            self.check(data,expected[page],17,19,page)
        # The decoder accepts no-op controls after exact expansion, but no data.
        data,expected,_=make(1,1,words=[0],transform=lambda b,j:b+b'\0'*1024)
        self.check(data,expected[0],1,1)
    def test_malformed_streams(self):
        good=encode([0x12345678],1)
        bad=[b'',b'\0'*8,good[:-1],good+b'\1\0',good+b'\x80\0',
             b'\2\x12\x34'+good[2:],b'\x80\x12'+good[2:],b'\xff\x12'+good[2:]]
        for wire in bad:
            data,_,_=make(1,1,transform=lambda b,j:wire)
            self.assertEqual(self.run_data(data)[0].returncode,2,wire)
        data,_,_=make(17,19)
        for n in [0,1,7,8,20,len(data)//2,len(data)-1]:
            self.assertEqual(self.run_data(data[:n])[0].returncode,2,n)
    def test_unsupported_metadata(self):
        fields=[(259,(3,[34677])),(262,(3,[32844])),(277,(3,[1])),(258,(3,[8]*3)),
                (339,(3,[1]*3)),(339,(3,[2,2])),(284,(3,[2])),(266,(3,[2])),
                (274,(3,[2])),(317,(3,[2])),(338,(3,[0]))]
        for key,value in fields:
            data,_,_=make(overrides={key:value});self.assertEqual(self.run_data(data)[0].returncode,2,(key,value))
    def test_libtiff_encoded_strips(self):
        encoder=ROOT/'target/logluv-encoder'
        self.assertTrue(encoder.exists(),'Run python3 scripts/upstream.py first')
        # Restored workspace artifacts can lose executable mode metadata.
        encoder.chmod(0o755)
        for w,h,rows,le,pattern in itertools.product([1,127,129],[1,5],[1,3],[False,True],['varied','runs']):
            _,expected,words=make(w,h,le,rows=rows,pattern=pattern)
            raw=self.path/'words.raw';raw.write_bytes(b''.join(struct.pack('<I',v) for v in words[0]))
            src=self.path/'encoded.tif'
            p=subprocess.run([str(encoder),str(raw),str(src),str(w),str(h),str(rows),'little' if le else 'big'],capture_output=True,timeout=30)
            self.assertEqual(p.returncode,0,p.stderr)
            self.check(src.read_bytes(),expected[0],w,h)
    def test_limits_and_original(self):
        src=ROOT/'tests/upstream/logluv-3c-16b.tiff';data=src.read_bytes()
        expected=native([0x000056c2]);meta=self.check(data,expected,1,1)
        self.assertEqual(expected,bytes.fromhex('0000011bb83c'))
        self.assertEqual((meta['scratch_bytes'],meta['output_bytes'],meta['decode_work']),(10,6,29))
        for flag,n in [('--max-output',9),('--max-input',len(data)-1),('--max-dimension',0),('--max-segments',0),('--work',meta['work_used']-1)]:
            p,_=self.run_data(data,flag,n);self.assertEqual(p.returncode,2);self.assertIn(b'Limit',p.stderr)
        self.assertEqual(self.run_data(data,'--work',meta['work_used'])[0].returncode,0)
        report=dict(file=src.name,input_sha256=hashlib.sha256(data).hexdigest(),pixel_sha256=hashlib.sha256(expected).hexdigest(),
            metadata=meta,native_components=struct.unpack('<hhh',expected),comparison='exact independent rational components and libtiff SGILOGDATAFMT_16BIT')
        (ROOT/'target/logluv-original-comparison.json').write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':unittest.main(verbosity=2)
