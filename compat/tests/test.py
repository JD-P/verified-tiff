"""C ABI contract tests and differential consumers of the pinned libtiff API."""
import itertools,json,os,pathlib,struct,subprocess,sys,tempfile,unittest
ROOT=pathlib.Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'tests'))
from extended import make_extended
from fixtures import make_tiff
from float64 import PATTERNS
OUT=pathlib.Path(os.environ.get('VTIFF_COMPAT_OUTPUT',ROOT/'target/compat')).resolve()
class CApi(unittest.TestCase):
 def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
 def tearDown(self):self.tmp.cleanup()
 def run_app(self,app,*args,env=None):return subprocess.run([str(OUT/app),*map(str,args)],capture_output=True,timeout=30,env=env)
 def compare(self,data,mode,expected=None,page=0,orientation=1,oracle=True):
  src=self.path/'input.tif';src.write_bytes(data);a=self.path/'ours.raw';b=self.path/'reference.raw'
  p=self.run_app('dump',src,mode,a,page,orientation);self.assertEqual(p.returncode,0,(mode,p.stderr))
  q=self.run_app('dump-libtiff',src,mode,b,page,orientation);self.assertEqual(q.returncode,0,(mode,q.stderr))
  if oracle:self.assertEqual(a.read_bytes(),b.read_bytes(),mode)
  if expected is not None:self.assertEqual(a.read_bytes(),expected)
  return a.read_bytes()
 def test_contracts_from_c(self):
  args=[ROOT/'tests/fixtures'/n for n in ['gray-strip.tif','logluv-pages.tif','rgb-be-tile.tif','alpha-4-1-none.tif']]+[ROOT/'tests/upstream/logluv-3c-16b.tiff']
  p=self.run_app('api-test',*args);self.assertEqual(p.returncode,0,p.stderr);print(p.stdout.decode().strip())
 def test_every_generated_stub(self):
  p=self.run_app('stub-test');self.assertEqual(p.returncode,0,p.stderr)
  api=json.loads((OUT/'api.json').read_text());print('C public API inventory:',len(api['functions']),'symbols;',sum(f['implementation']=='generated_stub' for f in api['functions']),'generated stubs exercised')
 def test_native_samples_and_byte_order(self):
  cases=0
  for (bits,ch,white),le,separate,compression in itertools.product([(8,1,False),(8,1,True),(8,3,False),(16,1,False),(16,1,True),(16,3,False),(64,1,False),(64,3,False)],[False,True],[False,True],[1,32773,5,8]):
   with self.subTest(bits=bits,ch=ch,le=le,separate=separate,compression=compression):
    data,_,native=make_extended(w=17,h=5,bits=bits,ch=ch,white=white,le=le,separate=separate,compression=compression,predictor=2 if bits==64 and compression in (5,8) else 1,sample_values=PATTERNS if bits==64 else None)
    size=bits//8;values=[native[0][i:i+size] for i in range(0,len(native[0]),size)]
    if bits==16 and sys.byteorder=='little':values=[v[::-1] for v in values]
    if bits==64 and sys.byteorder=='big':values=[v[::-1] for v in values]
    if separate:values=[values[p*ch+c] for c in range(ch) for p in range(17*5)]
    expected=b''.join(values)
    self.compare(data,'scan',expected);self.compare(data,'native',expected);cases+=1
  self.assertEqual(cases,128);print('C native matrix: 128 files, 256 exact scanline/strip comparisons including independent source samples')
 def test_bilevel_native_padding(self):
  for le,white,compression in itertools.product([False,True],[False,True],[1,32773,5,8]):
   data,_,native=make_extended(w=17,h=5,ch=1,bits=1,le=le,white=white,compression=compression)
   expected=bytearray()
   for y in range(5):
    row=bytearray(3)
    for x in range(17):row[x//8]|=native[0][y*17+x]<<(7-x%8)
    expected.extend(row)
   self.compare(data,'scan',bytes(expected));self.compare(data,'native',bytes(expected))
 def test_native_full_tiles(self):
  for bits,ch,le,separate,compression in itertools.product([8,16,64],[1,3],[False,True],[False,True],[1,5,8]):
   data,_,_=make_extended(w=32,h=32,bits=bits,ch=ch,le=le,tiled=True,separate=separate,compression=compression,sample_values=PATTERNS if bits==64 else None)
   self.compare(data,'native')
  print('C native full tiles: 72 exact libtiff comparisons')
 def test_rgba_rendering_and_orientations(self):
  opts=[dict(ch=1,bits=8),dict(ch=1,bits=8,white=True),dict(ch=3,bits=8),dict(ch=1,bits=16),dict(ch=1,bits=16,white=True),dict(ch=3,bits=16),dict(ch=1,bits=1,white=True),dict(ch=1,bits=4,palette=True),dict(ch=2,bits=8,alpha=2),dict(ch=4,bits=8,alpha=2)]
  cases=0
  for opt,le,tiled,compression in itertools.product(opts,[False,True],[False,True],[1,32773,5,8]):
   with self.subTest(opt=opt,le=le,tiled=tiled,compression=compression):
    data,pixels,_=make_extended(w=17,h=19,le=le,tiled=tiled,compression=compression,**opt)
    ch=3 if opt.get('palette') else opt['ch'];bits=8 if opt['bits']<8 else opt['bits'];step=ch*bits//8;expected=bytearray()
    for i in range(0,len(pixels[0]),step):
     p=pixels[0][i:i+step];rgb=[]
     for c in range(3):
      ci=0 if ch<=2 else c
      value=p[ci] if bits==8 else p[2*ci] if ch==1 else (int.from_bytes(p[2*ci:2*ci+2],'big')+128)//257
      rgb.append(value)
     alpha=p[-1] if ch in (2,4) else 255
     if ch==4:rgb=[(v*alpha+127)//255 for v in rgb]
     expected.extend(struct.pack('=I',rgb[0]|rgb[1]<<8|rgb[2]<<16|alpha<<24))
    # libtiff 4.7.1 put16bitbwtile advances pp by fromskew bytes,
    # not 2*fromskew; putagreytile has the same issue for Gray+A. Preserve
    # known-source pixels instead; native full tiles still compare exactly.
    known_oracle_bug=tiled and ((opt['ch']==1 and opt['bits']==16) or opt['ch']==2)
    self.compare(data,'rgba',bytes(expected),oracle=not known_oracle_bug);cases+=1
  self.assertEqual(cases,160)
  data,_,_=make_extended(w=7,h=5,bits=8)
  for orientation in [1,2,3,4]:self.compare(data,'rgba',orientation=orientation)
  print('C RGBA matrix: 160 independent-pixel checks, 136 exact libtiff comparisons; four output orientations')
 def test_raw_segments_pages_and_logluv(self):
  for le,tiled,page in itertools.product([False,True],[False,True],[0,1]):
   data,_,_=make_extended(w=17,h=19,ch=3,bits=16,le=le,tiled=tiled,compression=5,pages=2)
   self.compare(data,'raw',page=page)
  data=(ROOT/'tests/upstream/logluv-3c-16b.tiff').read_bytes();self.compare(data,'native',struct.pack('=hhh',0,6913,15544));self.compare(data,'scan',struct.pack('=hhh',0,6913,15544))
 def test_preserved_native_samples(self):
  for opts in [dict(ch=1,bits=4,palette=True),dict(ch=4,bits=8,alpha=1),dict(ch=2,bits=8,alpha=1)]:
   for le,tiled,codec in itertools.product([False,True],[False,True],[1,32773,5,8]):
    data,_,_=make_extended(w=17,h=19,le=le,tiled=tiled,compression=codec,**opts)
    self.compare(data,'native')
    if not tiled:self.compare(data,'scan')
    self.compare(data,'rgba',oracle=not (tiled and opts['ch']==2))
  print('Native palette, associated alpha and padded edge tiles: 48 files')
 def test_expected_unsupported_cases(self):
  cases=[(make_extended(ch=3,bits=64)[0],'rgba')]
  for data,mode in cases:
   src=self.path/'bad.tif';src.write_bytes(data);p=self.run_app('dump',src,mode,self.path/'bad.raw');self.assertNotEqual(p.returncode,0);self.assertGreater(p.returncode,0,p.stderr)
 def test_rgba_segments(self):
  for tiled,le,compression in itertools.product([False,True],[False,True],[1,5,8]):
   data,_,_=make_extended(w=17,h=19,ch=3,bits=8,tiled=tiled,le=le,compression=compression)
   self.compare(data,'rgba-segments')
 def test_mutated_inputs(self):
  seeds=sorted((ROOT/'tests/fixtures').glob('*.tif'))
  p=self.run_app('mutate-test',*seeds);self.assertEqual(p.returncode,0,p.stderr)
  print(p.stdout.decode().strip())
 def test_prelinked_binary_symbol_versions(self):
  app=OUT/'prelinked-dump'
  self.assertTrue(app.exists(),'prelinked libtiff.so.6 consumer must be built')
  src=ROOT/'tests/fixtures/gray-strip.tif';a=self.path/'abi.raw';b=self.path/'direct.raw'
  env=os.environ.copy();env['LD_LIBRARY_PATH']=str(OUT)
  p=self.run_app('prelinked-dump',src,'rgba',a,env=env);self.assertEqual(p.returncode,0,p.stderr)
  q=self.run_app('dump',src,'rgba',b);self.assertEqual(q.returncode,0,q.stderr);self.assertEqual(a.read_bytes(),b.read_bytes())
  print('Previously linked libtiff.so.6 consumer ran unchanged with compatibility library')
if __name__=='__main__':unittest.main(verbosity=2)
