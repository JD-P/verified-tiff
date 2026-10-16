"""CCITT lossless conformance, strict rejection and bounded compatibility."""
import itertools,json,subprocess,tempfile,unittest
from fax_fixtures import *
from extended import pixels
from fixtures import change_inline
BIN=ROOT/'target/debug/verified-tiff'
ORACLE=ROOT/'target/libtiff-oracle'
PROFILES=[dict(group=3),dict(group=3,aligned=True),dict(group=3,mixed=True),dict(group=3,mixed=True,aligned=True),dict(group=4),dict(group=3,missing=True,terminate=False)]
class Fax(unittest.TestCase):
    def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
    def tearDown(self):self.tmp.cleanup()
    def run_data(self,data,*flags):
        src=self.path/'in.tif';out=self.path/'out.pnm';src.write_bytes(data)
        p=subprocess.run([str(BIN),str(src),str(out),*map(str,flags)],capture_output=True,timeout=30)
        return p,out
    def check(self,opts,oracle=True):
        data,expected=make_fax(**opts);p,out=self.run_data(data);self.assertEqual(p.returncode,0,(opts,p.stderr));self.assertEqual(pixels(out),expected[0],opts)
        if oracle and ORACLE.exists():
            ref=self.path/'oracle.pnm';r=subprocess.run([str(ORACLE),str(self.path/'in.tif'),str(ref),'0'],capture_output=True,timeout=30)
            self.assertEqual(r.returncode,0,(opts,r.stderr));self.assertEqual(pixels(ref)[::3],expected[0],opts)
        return data,json.loads(p.stdout)
    def test_format_matrix(self):
        n=0
        for profile,le,tiled,white,fill,before in itertools.product(PROFILES,[False,True],[False,True],[False,True],[1,2],[False,True]):
            opts=dict(profile,le=le,tiled=tiled,white=white,fill=fill,before=before)
            with self.subTest(options=opts):self.check(opts);n+=1
        print(f'Fax format matrix: {n}; exact known pixels and independent libtiff rendering')
    def test_runs_edges_pages(self):
        n=0
        for profile,w,value in itertools.product(PROFILES,[1,7,8,9,63,64,65,1728,2560,3001],[0,255]):
            self.check(dict(profile,width=w,height=2,rows=4,pixel_value=lambda x,y,c,p,v=value:v));n+=1
        for profile in PROFILES:
            opts=dict(profile,pages=2,extra={258:None});data,expected=make_fax(**opts);p,out=self.run_data(data,'--page',1)
            self.assertEqual(p.returncode,0,p.stderr);self.assertEqual(pixels(out),expected[1]);n+=1
        print(f'Fax edge/long-run/page cases: {n}')
    def test_original_fixtures(self):
        report=[]
        for name in ['testfax3_bug54_1dnoEOL.tif','testfax3_bug_513.tiff','testfax4.tiff']:
            data=(ROOT/'.tools/tiff-4.7.1/test/images'/name).read_bytes();p,out=self.run_data(data);self.assertEqual(p.returncode,0,p.stderr)
            ref=self.path/'oracle.pnm';r=subprocess.run([str(ORACLE),str(self.path/'in.tif'),str(ref),'0'],capture_output=True,timeout=30)
            self.assertEqual(r.returncode,0,r.stderr);expected=pixels(ref)[::3];self.assertEqual(pixels(out),expected)
            report.append(dict(file=name,pixels=len(expected),exact=True,metadata=json.loads(p.stdout),oracle_warning=r.stderr.decode().strip()))
        (ROOT/'target/fax-original-comparison.json').write_text(json.dumps(report,indent=2)+'\n')
    def test_termination_and_zero_runs(self):
        for profile in PROFILES:
            self.check(dict(profile,terminate=False))
        # V0 against the initial all-white reference; EOFB optional.
        for tail in ['', '000000000001'*2]:
            p,out=self.run_data(explicit('1'+tail,13));self.assertEqual(p.returncode,0,p.stderr);self.assertEqual(pixels(out),bytes([255])*13)
        # Duplicate zero endpoints in the first reference row: initial H(0,0).
        # Next row's initial pass uses those endpoints; dropping them changes pixels.
        bits='001'+run(0,0)+run(0,1)+'001'+run(3,0)+run(4,1)+'1'
        bits+='0001'+'001'+run(2,0)+run(3,1)+'1'+'000000000001'*2
        p,out=self.run_data(explicit(bits,13,2));self.assertEqual(p.returncode,0,p.stderr)
        self.assertEqual(pixels(out),bytes([255]*3+[0]*4+[255]*6+[255]*2+[0]*3+[255]*8))
    def test_malformed_streams(self):
        # Illegal extension mode, incomplete horizontal, out-of-bounds vertical,
        # overlong run, early EOFB, bad terminal bits and excess trailing bytes.
        cases=['0000001','001','011','001'+run(14,0)+run(0,1),'000000000001'*2,'1'+'000000000001'*2+'1','1'+'000000000001'*2+'00000000']
        for bits in cases:
            p,_=self.run_data(explicit(bits,13));self.assertNotEqual(p.returncode,0,bits)
        # A mixed segment cannot begin with 2D; missing EOL recovery is 1D only.
        p,_=self.run_data(explicit('0000000000010'+'1',13,group=3,extra={292:(4,[1])}));self.assertNotEqual(p.returncode,0)
        for opts in [dict(group=3,mixed=True,missing=True),dict(group=3,aligned=True,missing=True)]:
            p,_=self.run_data(make_fax(**opts)[0]);self.assertNotEqual(p.returncode,0,opts)
        data=make_fax(width=7,height=1,group=3)[0]
        for tag,val in [(292,2),(292,8),(258,8),(259,2),(266,3),(274,2),(284,2),(317,2)]:
            try:bad=change_inline(data,tag,val)
            except (KeyError,AssertionError):continue
            self.assertNotEqual(self.run_data(bad)[0].returncode,0,(tag,val))
    def test_independent_libtiff_encoder(self):
        from fixtures import make_tiff
        n=0;cp=ROOT/'.tools/tiff-4.7.1/tools/tiffcp'
        if not cp.exists():self.skipTest('libtiff tiffcp is not built')
        for mode,fill,tiled,white in itertools.product(['g3:1d','g3:1d:fill','g3:2d','g3:2d:fill','g4'],[1,2],[False,True],[False,True]):
            data,expected=make_tiff(width=37,height=29,bits=1,tiled=tiled,white=white,rows=7)
            source=self.path/'source.tif';source.write_bytes(data);encoded=self.path/'encoded.tif'
            cmd=[str(cp),'-c',mode,'-f','lsb2msb' if fill==2 else 'msb2lsb',str(source),str(encoded)]
            p=subprocess.run(cmd,capture_output=True,timeout=20);self.assertEqual(p.returncode,0,p.stderr)
            p,out=self.run_data(encoded.read_bytes());self.assertEqual(p.returncode,0,(mode,fill,tiled,white,p.stderr));self.assertEqual(pixels(out),expected[0]);n+=1
        print(f'Fax independent libtiff encodings: {n}; exact known pixels')
    def test_resources(self):
        data,meta=self.check(dict())
        for flags in [('--work',0),('--work',meta['work_used']-1),('--max-output',meta['output_bytes']-1),('--max-input',len(data)-1),('--max-dimension',16),('--max-tags',1),('--max-segments',1),('--page',1)]:
            self.assertNotEqual(self.run_data(data,*flags)[0].returncode,0,flags)
        self.assertEqual(self.run_data(data,'--work',meta['work_used'])[0].returncode,0)

if __name__=='__main__':unittest.main()
