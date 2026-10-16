"""JPEG/OJPEG conformance and independent TIFF/JPEG decoder comparisons."""
import itertools,json,pathlib,struct,subprocess,tempfile,unittest
from jpeg_fixtures import *
from extended import pixels
from fixtures import change_inline
BIN=ROOT/'target/debug/verified-tiff'
PROFILES=[dict(ch=1,hs=1,vs=1),dict(ch=3,hs=1,vs=1,photo=2)]+[dict(ch=3,hs=h,vs=v) for h,v in [(1,1),(2,1),(1,2),(2,2),(4,1),(1,4)]]
class JPEG(unittest.TestCase):
    def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
    def tearDown(self):self.tmp.cleanup()
    def run_data(self,data,*flags):
        src=self.path/'in.tif';out=self.path/'ours.pnm';src.write_bytes(data)
        p=subprocess.run([str(BIN),str(src),str(out),*map(str,flags)],capture_output=True,timeout=20);return p,out
    def check(self,opts,compare=True):
        references=[];data=make_jpeg_tiff(**opts,references=references);p,out=self.run_data(data);self.assertEqual(p.returncode,0,(opts,p.stderr))
        if compare:
            ref=generated_oracle(opts,references,self.path);a=pixels(out);self.assertEqual(len(a),len(ref))
            diff=[abs(x-y) for x,y in zip(a,ref)];self.assertLessEqual(max(diff),3,(opts,max(diff)));self.assertLessEqual(sum(diff)/len(diff),0.5,(opts,'mean error'))
        return data,json.loads(p.stdout)
    def test_format_matrix(self):
        n=0
        for profile,le,tiled,mode,restart,quality in itertools.product(PROFILES,[False,True],[False,True],['self','shared','old-tables'],[0,1],[30,95]):
            options=dict(profile,le=le,tiled=tiled,mode=mode,restart=restart,quality=quality)
            with self.subTest(options=options):self.check(options);n+=1
        print(f'JPEG matrix: {n}; independent libjpeg native components; maximum channel error <=3')
    def test_legacy_headers_and_range(self):
        n=0
        for profile,le,restart in itertools.product(PROFILES,[False,True],[0,3]):
            opts=dict(profile,le=le,restart=restart,mode='old-header',holes=True)
            with self.subTest(options=opts):self.check(opts);n+=1
        for mode,tiled,le in itertools.product(['self','shared','old-tables','old-header'],[False,True],[False,True]):
            if mode=='old-header' and tiled:continue
            self.check(dict(mode=mode,tiled=tiled,le=le,limited=True));n+=1
        print(f'JPEG legacy/header/reference-range matrix: {n}')
    def test_edges_and_optimized_tables(self):
        n=0
        for profile,(w,h),mode in itertools.product(PROFILES,[(1,1),(7,9),(8,8),(9,7),(31,33)],['self','shared','old-tables']):
            opts=dict(profile,w=w,h=h,mode=mode,optimize=mode=='self')
            with self.subTest(options=opts):self.check(opts);n+=1
        print(f'JPEG edge/optimized-table matrix: {n}')
    def test_original_fixtures(self):
        names=['quad-tile.jpg.tiff','ojpeg_chewey_subsamp21_multi_strip.tiff','ojpeg_zackthecat_subsamp22_single_strip.tiff','ojpeg_single_strip_no_rowsperstrip.tiff']
        report=[]
        for name in names:
            src=ROOT/'.tools/tiff-4.7.1/test/images'/name;p,out=self.run_data(src.read_bytes());self.assertEqual(p.returncode,0,p.stderr)
            expected=oracle(src,self.path/'reference.raw');actual=pixels(out);diff=[abs(a-b) for a,b in zip(actual,expected)]
            self.assertEqual(len(actual),len(expected));self.assertLessEqual(max(diff),3);self.assertLess(sum(diff)/len(diff),0.1)
            report.append(dict(file=name,max_channel_error=max(diff),mean_absolute_error=sum(diff)/len(diff),samples=len(diff)))
        (ROOT/'target/jpeg-original-comparison.json').write_text(json.dumps(report,indent=2)+'\n')
    def test_resources_and_container_rejections(self):
        data,meta=self.check(dict())
        for flags in [('--work',0),('--work',meta['work_used']-1),('--max-output',meta['scratch_bytes']-1),('--max-input',len(data)-1),('--max-dimension',16),('--max-tags',1),('--max-segments',1),('--page',1)]:
            p,_=self.run_data(data,*flags);self.assertNotEqual(p.returncode,0,flags)
        p,_=self.run_data(data,'--work',meta['work_used']);self.assertEqual(p.returncode,0,p.stderr)
        for tag,value in [(259,99),(258,12),(274,2),(277,4),(284,2),(317,2),(512,2)]:
            # Change only existing fields; add optional unsupported tags through IFD edits below.
            try:bad=change_inline(data,tag,value)
            except (KeyError,AssertionError):continue
            p,_=self.run_data(bad);self.assertNotEqual(p.returncode,0,(tag,value))
    def test_malformed_markers_and_entropy(self):
        data=make_jpeg_tiff(mode='self',ch=1,w=8,h=8)
        # Locate the actual strip via its directory, not byte-pattern guesses.
        le=data[:2]==b'II';order='<' if le else '>';off=struct.unpack_from(order+'I',data,4)[0];n=struct.unpack_from(order+'H',data,off)[0]
        tags={struct.unpack_from(order+'H',data,off+2+12*j)[0]:off+2+12*j for j in range(n)}
        start=struct.unpack_from(order+'I',data,tags[273]+8)[0];count=struct.unpack_from(order+'I',data,tags[279]+8)[0]
        jpg=data[start:start+count];sos=jpg.index(b'\xff\xda');sof=jpg.index(b'\xff\xc0');dqt=jpg.index(b'\xff\xdb');dht=jpg.index(b'\xff\xc4')
        changes=[(0,0),(sof+1,194),(sof+4,12),(sos+6,255),(dqt+5,0),(dht+5,255),(count-1,0),(count-2,0)]
        for at,v in changes:
            b=bytearray(data);b[start+at]=v;p,_=self.run_data(b);self.assertNotEqual(p.returncode,0,(at,v))
        for cut in [1,2,3,10,count//2,count-1]:
            b=bytearray(data);struct.pack_into(order+'I',b,tags[279]+8,cut);p,_=self.run_data(b);self.assertNotEqual(p.returncode,0,cut)
        # Extra data following EOI is rejected even when the TIFF byte count includes it.
        b=bytearray(data);struct.pack_into(order+'I',b,tags[279]+8,count+1);p,_=self.run_data(b);self.assertNotEqual(p.returncode,0)
    def test_directory_compatibility_and_padding(self):
        data=make_jpeg_tiff(ch=1,mode='old-header',holes=True)
        self.assertEqual(self.run_data(data)[0].returncode,0)
        off=int.from_bytes(data[4:8],'little');n=int.from_bytes(data[off:off+2],'little')
        holes=[off+2+12*i for i in range(n) if data[off+2+12*i:off+14+12*i]==bytes(12)]
        self.assertEqual(len(holes),2)
        # A tombstone is accepted only when every byte is zero.
        bad=bytearray(data);bad[holes[0]+8]=1;self.assertNotEqual(self.run_data(bad)[0].returncode,0)
        # Duplicating a recognized field never silently selects one value.
        bad=bytearray(data);bad[holes[0]:holes[0]+12]=data[off+2:off+14]
        self.assertNotEqual(self.run_data(bad)[0].returncode,0)
        # A model witness with a non-one modern padding bit is invalid.
        data=bytearray((ROOT/'tests/fixtures/jpeg-witness.tif').read_bytes());data[268]=0x3e
        self.assertNotEqual(self.run_data(data)[0].returncode,0)
        # Unstuffed FF cannot be consumed as coefficient entropy.
        data[268]=255;self.assertNotEqual(self.run_data(data)[0].returncode,0)

    def test_restart_sequence(self):
        data=make_jpeg_tiff(mode='self',w=31,h=17,restart=1);markers=[i for i in range(len(data)-1) if data[i]==255 and 208<=data[i+1]<=215];self.assertTrue(markers)
        for at in markers[:3]:
            b=bytearray(data);b[at+1]=208+(b[at+1]-208+1)%8;p,_=self.run_data(b);self.assertNotEqual(p.returncode,0)

if __name__=='__main__':unittest.main()
