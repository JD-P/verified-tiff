"""Independent Zstandard streams, pinned encoder comparisons and invalid frames."""
import collections,json,pathlib,random,subprocess,tempfile,time,unittest
from zstd_reference import compress,decompress,LIB,ROOT
EXE=ROOT/'target/release/examples/zstd-native'
def frame(blocks):
 # No content-size field: a 1 KiB window also permits tiny outputs with trees
 # whose encoded descriptions are larger than their decoded data.
 out=bytearray.fromhex('28b52ffd0000')
 for i,(kind,size,data) in enumerate(blocks):out.extend(((size<<3)|(kind<<1)|(i==len(blocks)-1)).to_bytes(3,'little'));out.extend(data)
 return bytes(out)
def manual_streams():
 # Weight 1 for A, inferred weight 1 for B. All earlier symbols have weight 0.
 description=bytes([193])+bytes(32)+b'\x01'
 def literals(n,payload,kind=2,four=False):
  return ((len(payload)<<14)|(n<<4)|kind|(4 if four else 0)).to_bytes(3,'little')+payload+b'\0'
 one=literals(4,description+b'\x16');reuse=literals(4,b'\x16',3)
 four=literals(8,description+b'\x01\0\x01\0\x01\0\x05\x06\x05\x06',four=True)
 repeated=b'\x45\x06A\0' # RLE literals: 100 A bytes, no sequences.
 first=b'\x18abc\x01\x54\x03\x02\0\x06'
 second=b'\x18abc\x01\xfc\x06' # Reuse the three RLE FSE tables.
 return [(frame([(2,len(one),one)]),b'ABBA'),(frame([(2,len(one),one),(2,len(reuse),reuse)]),b'ABBAABBA'),(frame([(2,len(four),four)]),b'ABBAABBA'),(frame([(2,len(repeated),repeated)]),b'A'*100),(frame([(2,len(first),first),(2,len(second),second)]),b'abcabcabcabc')]
def features(s):
 result=collections.Counter();f=s[4];single=bool(f&32);fl=[int(single),2,4,8][f>>6];at=5+(not single)+[0,1,2,4][f&3]+fl
 while True:
  v=int.from_bytes(s[at:at+3],'little');at+=3;kind=(v>>1)&3;size=v>>3;result[f'block:{kind}']+=1
  if kind==2:
   b=s[at];lt=b&3;fmt=(b>>2)&3;result[f'literals:{lt}']+=1
   if lt<2:
    n=[1,2,1,3][fmt];regen=int.from_bytes(s[at:at+n],'little')>>(3 if n==1 else 4);stored=regen if lt==0 else 1
   else:
    n=3 if fmt<2 else fmt+2;bits=[10,10,14,18][fmt];stored=int.from_bytes(s[at:at+n],'little')>>(4+bits)
    if lt==2:result['weights:direct' if s[at+n]>=128 else 'weights:fse']+=1
   q=at+n+stored;num=s[q];q+=1
   if num>=128:q+=1 if num<255 else 2
   if num:
    mode=s[q]
    for name,shift in [('ll',6),('of',4),('ml',2)]:result[f'{name}:{(mode>>shift)&3}']+=1
  at+=1 if kind==1 else size
  if v&1:break
 return result
class Zstandard(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  if not LIB.exists():subprocess.run(['python3',str(ROOT/'scripts/zstd-dependency.py')],check=True)
  if not EXE.exists():raise RuntimeError('Build the zstd-native release example first')
 def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.file=pathlib.Path(self.tmp.name)/'frame.zst'
 def tearDown(self):self.tmp.cleanup()
 def run_frame(self,s,needed,cap):
  self.file.write_bytes(s);return subprocess.run([str(EXE),str(self.file),str(needed),str(cap)],capture_output=True,timeout=30)
 def test_pinned_reference_matrix(self):
  rng=random.Random(71027);counts=collections.Counter();cases=0;start=time.monotonic()
  for n in [0,1,3,31,32,127,255,256,1024,32768,131072,131073,300000]:
   for pattern in ['constant','random','small','repeat']:
    if pattern=='constant':data=b'A'*n
    elif pattern=='random':data=rng.randbytes(n)
    elif pattern=='small':data=bytes(rng.randrange(13) for _ in range(n))
    else:data=(rng.randbytes(1473)*(n//1473+1))[:n]
    for level in [-3,1,3,9,19]:
     source=compress(data,level,content_size=bool(cases%2),checksum=bool((cases//2)%2));counts.update(features(source))
     r=self.run_frame(source,n,n)
     with self.subTest(n=n,pattern=pattern,level=level):self.assertEqual(r.returncode,0,r.stderr);self.assertEqual(r.stdout,data)
     cases+=1
  for source,expected in manual_streams():counts.update(features(source))
  self.assertTrue(all(counts[k]>0 for k in ['block:0','block:1','block:2','literals:0','literals:1','literals:2','literals:3','weights:direct','weights:fse','ll:0','ll:1','ll:2','ll:3','of:0','of:1','of:2','of:3','ml:0','ml:1','ml:2','ml:3']),dict(counts))
  report={'reference':'Zstandard 1.5.7','frames':cases,'feature_counts':dict(counts),'seconds':time.monotonic()-start}
  (ROOT/'target/zstd-comparison.json').write_text(json.dumps(report,indent=2)+'\n');print('Zstandard:',cases,'reference-encoded frames, all block/literal/table modes exercised')
 def test_independent_literal_and_table_streams(self):
  for source,expected in manual_streams():
   self.assertEqual(decompress(source,len(expected)),expected)
   r=self.run_frame(source,len(expected),len(expected));self.assertEqual(r.returncode,0,r.stderr);self.assertEqual(r.stdout,expected)
 def test_all_truncations_and_strict_trailing_policy(self):
  examples=manual_streams()+[(compress(bytes(range(256))*4,checksum=True),bytes(range(256))*4)]
  for source,expected in examples:
   for i in range(len(source)):
    r=self.run_frame(source[:i],len(expected),len(expected));self.assertNotEqual(r.returncode,0,(len(source),i))
   for tail in [b'\0',b'junk',source]:self.assertNotEqual(self.run_frame(source+tail,len(expected),2*len(expected)).returncode,0)
 def test_deterministic_mutations_match_reference_when_accepted(self):
  rng=random.Random(6027);seeds=manual_streams()+[(compress(bytes(rng.randrange(13) for _ in range(4096))),None)]
  accepted=0
  for i in range(1500):
   source,expected=seeds[i%len(seeds)];b=bytearray(source)
   for _ in range(1+i%3):b[rng.randrange(len(b))]^=1<<rng.randrange(8)
   source=bytes(b);r=self.run_frame(source,0,8192)
   if r.returncode==0:self.assertEqual(r.stdout,decompress(source,8192));accepted+=1
  print('Zstandard mutations: 1500, accepted comparisons:',accepted)
 def test_checksum_content_size_reserved_and_dictionary(self):
  data=b'the checksum covers the complete decoded frame'*43;source=compress(data,checksum=True)
  for index in range(len(source)-4,len(source)):
   b=bytearray(source);b[index]^=1;self.assertNotEqual(self.run_frame(bytes(b),1,len(data)).returncode,0)
  for flags in [8,1]:
   b=bytearray(source);b[4]|=flags
   if flags==1:b.insert(5,7)
   self.assertNotEqual(self.run_frame(bytes(b),1,len(data)).returncode,0)
  self.assertNotEqual(self.run_frame(source,len(data),len(data)-1).returncode,0)
  r=self.run_frame(source,1,len(data)+23);self.assertEqual(r.returncode,0,r.stderr);self.assertEqual(r.stdout,data)
if __name__=='__main__':unittest.main(verbosity=2)
