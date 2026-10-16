"""Independent native samples, BigTIFF/SubIFDs, seekable I/O and options.
Generated with Python struct/zlib and independent TIFF LZW/PackBits encoders;
never encoded by the implementation under test. Native bytes also compared
against pinned libtiff 4.7.1 where its format/predictor supports them.
"""
import itertools,pathlib,struct,subprocess,sys,tempfile,unittest,zlib,os
ROOT=pathlib.Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'tests'))
from fixtures import lzw_encode,packbits_row
from extended import pixels
OUT=pathlib.Path(os.environ.get('VTIFF_COMPAT_OUTPUT',ROOT/'target/compat')).resolve()
WIDTHS={1:1,2:1,3:2,4:4,5:8,7:1,11:4,12:8,13:4,16:8,17:8,18:8}
def assemble(pages,big=False,le=True,main_next=None,origin=0,dedup=False):
 e='<' if le else '>';hs=16 if big else 8;cw=8 if big else 2;ew=20 if big else 12;slot=8 if big else 4
 q='Q' if big else 'I';dirs=[];at=hs
 for tags in pages:dirs.append(at+origin);at+=cw+ew*len(tags)+slot
 out=bytearray(at);out[:2]=b'II' if le else b'MM'
 struct.pack_into(e+'H',out,2,43 if big else 42)
 if big:struct.pack_into(e+'HHQ',out,4,8,0,dirs[0])
 else:struct.pack_into(e+'I',out,4,dirs[0])
 patches=[];locations={};encoded_positions=[]
 for pi,tags in enumerate(pages):
  off=dirs[pi]-origin;struct.pack_into(e+('Q' if big else 'H'),out,off,len(tags))
  nxt=main_next[pi] if main_next else None
  struct.pack_into(e+q,out,off+cw+ew*len(tags),dirs[nxt] if nxt is not None else 0)
  for i,(key,ty,vals) in enumerate(sorted(tags)):
   if isinstance(vals,tuple) and vals[0]=='SUB': vals=[dirs[n] for n in vals[1]]
   segs=vals[1] if isinstance(vals,tuple) and vals[0]=='SEG' else None
   if segs is not None: vals=[0]*len(segs)
   if isinstance(vals,bytes):raw=vals;n=len(raw)//WIDTHS[ty]
   else:
    n=len(vals);fmt={1:'B',3:'H',4:'I',13:'I',16:'Q',17:'q',18:'Q'}[ty];raw=struct.pack(e+str(n)+fmt,*vals)
   p=off+cw+ew*i;struct.pack_into(e+'HH'+q,out,p,key,ty,n)
   loc=p+ew-slot
   if len(raw)>slot:
    loc=len(out);struct.pack_into(e+q,out,p+ew-slot,loc+origin);out.extend(raw)
   else:out[loc:loc+len(raw)]=raw
   locations[(pi,key)]=(loc,ty,n)
   if segs is not None:patches.append((loc,ty,segs))
 cache={}
 for loc,ty,segs in patches:
  for i,seg in enumerate(segs):
   if dedup and seg in cache:pos=cache[seg]
   else:pos=len(out)+origin;out.extend(seg);cache[seg]=pos
   struct.pack_into(e+('Q' if ty==16 else 'I'),out,loc+i*WIDTHS[ty],pos);encoded_positions.append(pos)
 return bytes(out),locations,encoded_positions

def scene(w=7,h=5,ch=3,bits=8,fmt=1,codec=1,predict=1,le=True,big=False,tiled=False,separate=False,alpha=0,orientation=1,photo=None,extra=(),origin=0,fill=1):
 e='<' if le else '>';mask=(1<<bits)-1;bw=16 if tiled else w;bh=16 if tiled else 3
 across=(w+bw-1)//bw;down=(h+bh-1)//bh
 def val(x,y,c):
  if fmt==3:
   p=[0,1<<(bits-1), ((1<<(bits-1))-1), (0x7fc01234 if bits==32 else 0x7ff812345678abcd), (0x3f800000 if bits==32 else 0x3ff0000000000000)]
   return p[(x+3*y+c)%len(p)]
  return (x*1013+y*3137+c*9109+(mask if (x+y+c)%3==0 else 0))&mask
 segments=[];native=[];scan=[]
 for plane in range(ch if separate else 1):
  for by in range(down):
   for bx in range(across):
    rr=bh if tiled else min(bh,h-by*bh);rows=[];host=[]
    for yy in range(rr):
     words=[val(bx*bw+x,by*bh+yy,plane if separate else c) for x in range(bw) for c in range(1 if separate else ch)]
     nc=1 if separate else ch
     if bits<8:
      raw=bytearray((len(words)*bits+7)//8)
      for i,v in enumerate(words):raw[i*bits//8]|=v<<(8-bits-(i*bits)%8)
      # Deliberately nonzero unused bits, preserved by native APIs.
      if len(words)*bits%8:raw[-1]|=(1<<(8-len(words)*bits%8))-1
      host.append(bytes(raw));rows.append(bytes(raw))
     else:
      host.append(b''.join(v.to_bytes(bits//8,sys.byteorder) for v in words))
      delta=[(v-(words[i-nc] if i>=nc else 0))&mask for i,v in enumerate(words)] if predict==2 else words
      if predict==3:
       bps=bits//8;shuffled=bytes(v.to_bytes(bps,'big')[j] for j in range(bps) for v in words)
       rows.append(bytes((v-(shuffled[i-nc] if i>=nc else 0))&255 for i,v in enumerate(shuffled)))
      else:rows.append(b''.join(v.to_bytes(bits//8,'little' if le else 'big') for v in delta))
    native.append(b''.join(host));raw=b''.join(rows)
    if codec==50000:
     from zstd_reference import compress
     encoded=compress(raw)
    else:encoded=raw if codec==1 else b''.join(packbits_row(r) for r in rows) if codec==32773 else lzw_encode(raw) if codec==5 else zlib.compress(raw)
    if fill==2:encoded=bytes(int(f'{v:08b}'[::-1],2) for v in encoded)
    segments.append(encoded)
  if not tiled:scan.extend(native[plane*down:(plane+1)*down])
 photo=(2 if ch>=3 else 1) if photo is None else photo
 ot=16 if big else 4
 tags=[(256,4,[w]),(257,4,[h]),(258,3,[bits]*ch),(259,3,[codec]),(262,3,[photo]),(266,3,[fill]),(274,3,[orientation]),(277,3,[ch]),(284,3,[2 if separate else 1]),(317,3,[predict]),(339,3,[fmt]*ch),((324 if tiled else 273),ot,('SEG',segments)),((325 if tiled else 279),ot,list(map(len,segments)))]
 tags+= [(322,4,[bw]),(323,4,[bh])] if tiled else [(278,4,[bh])]
 if alpha:tags.append((338,3,[alpha]))
 tags+=list(extra)
 blob,loc,pos=assemble([tags],big,le,origin=origin)
 return blob,b''.join(native),b''.join(scan),tags,loc,pos

class Coverage(unittest.TestCase):
 def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
 def tearDown(self):self.tmp.cleanup()
 def runapp(self,name,*args):return subprocess.run([str(OUT/name),*map(str,args)],capture_output=True,timeout=60)
 def compare(self,blob,mode,expected,oracle=True,orientation=1):
  p=self.path/'in.tif';p.write_bytes(blob);a=self.path/'a.bin';b=self.path/'b.bin'
  r=self.runapp('dump',p,mode,a,0,orientation);self.assertEqual(r.returncode,0,r.stderr);self.assertEqual(a.read_bytes(),expected)
  if oracle:
   q=self.runapp('dump-libtiff',p,mode,b,0,orientation);self.assertEqual(q.returncode,0,q.stderr);self.assertEqual(b.read_bytes(),expected)
 def contract(self,mode,blob,*args):
  p=self.path/'contract.tif';p.write_bytes(blob);r=self.runapp('coverage-test',mode,p,*args);self.assertEqual(r.returncode,0,r.stderr)
 def test_native_word_formats_prediction_alpha_and_edges(self):
  cases=0;oracle_cases=0
  for bits,fmt,ch,alpha in [(8,2,1,0),(16,2,3,0),(32,2,1,0),(64,2,3,0),(32,3,1,0),(64,3,3,0),(16,1,4,1),(16,1,4,2),(32,1,4,1),(64,1,4,2)]:
   for le,big,separate,tiled,codec in itertools.product([False,True],[False,True],[False,True],[False,True],[1,32773,5,8]):
    pred=2 if codec in (5,8) else 1
    blob,native,scan,*_=scene(w=17,h=19,ch=ch,bits=bits,fmt=fmt,alpha=alpha,le=le,big=big,separate=separate,tiled=tiled,codec=codec,predict=pred)
    # libtiff Predictor 2 rejects 64-bit words; bitwise modular transport is
    # covered by an independent source expectation and the Verus theorem.
    oracle=not (bits==64 and pred==2)
    with self.subTest(bits=bits,fmt=fmt,ch=ch,le=le,big=big,separate=separate,tiled=tiled,codec=codec):
     self.compare(blob,'native',native,oracle)
     if not tiled:self.compare(blob,'scan',scan,oracle)
    cases+=1;oracle_cases+=oracle
  print(f'Broader native samples: {cases} independent files, {oracle_cases} pinned-oracle files')
 def test_all_source_and_output_orientations(self):
  for source,out in itertools.product(range(1,9),repeat=2):
   blob,native,*_=scene(w=7,h=5,ch=3,orientation=source,codec=8,predict=2)
   pixels=[native[i:i+3]+b'\xff' for i in range(0,len(native),3)]
   def f(o):o=(o-1)%4+1;return o in (2,3),o in (3,4)
   a,b=f(source),f(out);expected=[b'']*35
   for y in range(5):
    for x in range(7):expected[(4-y if a[1]!=b[1] else y)*7+(6-x if a[0]!=b[0] else x)]=pixels[y*7+x]
   expected=b''.join(struct.pack('=I',int.from_bytes(p,'little')) for p in expected)
   self.compare(blob,'rgba',expected,orientation=out)
 def test_rgba16_alpha_scaling(self):
  for le,big,separate,tiled,alpha in itertools.product([False,True],[False,True],[False,True],[False,True],[1,2]):
   blob,*_=scene(w=17,h=19,ch=4,bits=16,alpha=alpha,le=le,big=big,separate=separate,tiled=tiled,codec=8,predict=2)
   expected=bytearray()
   for y in range(19):
    for x in range(17):
     v=[((x*1013+y*3137+c*9109+(65535 if (x+y+c)%3==0 else 0))&65535) for c in range(4)]
     rgba=[(a+128)//257 for a in v]
     if alpha==2:rgba[:3]=[(a*rgba[3]+127)//255 for a in rgba[:3]]
     expected.extend(struct.pack('=I',int.from_bytes(bytes(rgba),'little')))
   self.compare(blob,'rgba',bytes(expected))
 def test_segment_source_orientations_and_edge_padding(self):
  for tiled,orientation in itertools.product([False,True],range(1,9)):
   blob,native,*_=scene(w=17,h=19,ch=3,tiled=tiled,orientation=orientation,codec=8)
   bw,bh=(16,16) if tiled else (17,3);at=0;expected=bytearray();o=(orientation-1)%4+1;fx=o in (2,3);fy=o not in (3,4)
   for y in range(0,19,bh):
    for x in range(0,17,bw):
     hh=bh if tiled else min(bh,19-y);ww=min(bw,17-x);rh=min(hh,19-y);block=native[at:at+bw*hh*3];at+=len(block);out=bytearray(bw*hh*4)
     for dy in range(rh):
      for dx in range(ww):
       tx=ww-1-dx if fx else dx;ty=hh-rh+(rh-1-dy if fy else dy);p=block[(dy*bw+dx)*3:(dy*bw+dx)*3+3]+b'\xff';out[(ty*bw+tx)*4:(ty*bw+tx)*4+4]=struct.pack('=I',int.from_bytes(p,'little'))
     expected.extend(out)
   self.compare(blob,'rgba-segments',bytes(expected))
 def test_packed_fill_order(self):
  for bits,fill,codec in itertools.product([1,2,4],[1,2],[1,32773,5,8]):
   blob,native,scan,*_=scene(bits=bits,ch=1,fill=fill,codec=codec,w=17)
   self.compare(blob,'native',native);self.compare(blob,'scan',scan)
 def test_common_metadata_and_read_modes(self):
  for le,big in itertools.product([False,True],[False,True]):
   e='<' if le else '>'
   extra=[(282,5,struct.pack(e+'II',300,1)),(283,5,struct.pack(e+'II',1440,5)),(296,3,[2]),(297,3,[7,12]),(270,2,b'Image description\0'),(305,2,b'Verified TIFF tests\0'),(315,2,b'Author\0'),(34675,7,b'\0ICC\xff\0profile'),(700,1,b'<xmp/>'),(65000,2,b'unknown\0')]
   self.contract('metadata',scene(ch=1,extra=extra,le=le,big=big)[0],int(big))
  blob,*_=scene(w=4096,h=10,ch=1)
  # Explicit single strip for chopping (32 rows, 4-byte rows -> 128 bytes)
  tags=scene(w=4096,h=2,ch=1)[3];tags=[t for t in tags if t[0] not in (257,278,273,279)]+[(257,4,[10]),(278,4,[10]),(273,4,('SEG',[bytes(range(256))*160])),(279,4,[40960])]
  blob=assemble([tags])[0];self.contract('chop',blob)
 def test_lazy_callbacks_and_bounded_memory(self):
  tags=scene(w=4096,h=3,ch=1,codec=8)[3];good=zlib.compress(bytes([57])*4096);bad=b'bad!'
  tags=[t for t in tags if t[0] not in (257,278,273,279)]+[(257,4,[8192]),(278,4,[1]),(273,4,('SEG',[good]+[bad]*8191)),(279,4,[len(good)]+[len(bad)]*8191)]
  blob,_,pos=assemble([tags],dedup=True)
  self.contract('seek',blob,min(pos))
  blob,*_=scene(w=4096,h=3,ch=1,codec=1);self.contract('limits',blob)
 def test_work_budget_is_shared_across_rgba_segments(self):
  blob,_,_,tags,*_=scene(codec=8,predict=2)
  counts=next(v for key,_,v in tags if key==279)
  work=sum(8*n+3*63+64 for n in counts)-1
  self.contract('work',blob,work)
 def test_bigtiff_subifds_and_sparse_offsets(self):
  for big,le in itertools.product([False,True],[False,True]):
   tags=scene(ch=1,big=big,le=le)[3];child=scene(w=3,h=2,ch=1,big=big,le=le)[3]
   tags=tags+[(330,18 if big else 13,('SUB',[1]))]
   blob,_,_=assemble([tags,child],big,le);self.contract('subifd',blob,int(big))
  for le in [False,True]:
   origin=(1<<32)+4096;blob,native,*_=scene(ch=1,big=True,le=le,origin=origin)
   p=self.path/'sparse.tif'
   with p.open('wb') as f:f.write(blob[:16]);f.seek(origin+16);f.write(blob[16:])
   r=self.runapp('coverage-test','sparse',p);self.assertEqual(r.returncode,0,r.stderr)
   self.assertEqual(r.stdout,native)
 def test_malformed_bigtiff_and_directory_rollback(self):
  blob,_,_,tags,loc,_=scene(big=True,ch=1)
  for at,fmt,v in [(4,'H',4),(6,'H',1),(8,'Q',(1<<64)-8),(16,'Q',(1<<64)-1)]:
   bad=bytearray(blob);struct.pack_into('<'+fmt,bad,at,v);self.contract('bad',bad)
  # Huge LONG8 payload length and range overflow in a real directory record.
  off=16+8;bad=bytearray(blob);struct.pack_into('<HHQQ',bad,off,256,16,(1<<64)-1,(1<<64)-16);self.contract('bad',bad)
  cyclic=bytearray(blob);struct.pack_into('<Q',cyclic,16+8+20*len(tags),16);self.contract('cycle',cyclic)
 def test_special_codecs_originals_remain_segment_readable(self):
  names=['ojpeg_single_strip_no_rowsperstrip.tiff','ojpeg_zackthecat_subsamp22_single_strip.tiff','ojpeg_chewey_subsamp21_multi_strip.tiff','quad-tile.jpg.tiff','webp_lossless_rgba_alpha_fully_opaque.tif','logluv-3c-16b.tiff']
  # Lossy original JPEGs are compared with the already tested Rust raster's
  # specified transform elsewhere. Here require all individual segments work.
  for name in names:
   p=ROOT/'tests/upstream'/name;r=self.runapp('dump',p,'native',self.path/'native.bin');self.assertEqual(r.returncode,0,(name,r.stderr))
   if name.startswith('ojpeg'):
    q=subprocess.run([str(ROOT/'target/debug/verified-tiff'),str(p),str(self.path/'core.pnm')],capture_output=True,timeout=60)
    self.assertEqual(q.returncode,0,q.stderr)
    actual=(self.path/'native.bin').read_bytes()
    data=p.read_bytes();e='<' if data[:2]==b'II' else '>';off=struct.unpack_from(e+'I',data,4)[0];tags={}
    for i in range(struct.unpack_from(e+'H',data,off)[0]):
     at=off+2+i*12;key,ty,count=struct.unpack_from(e+'HHI',data,at)
     if count==1 and ty in (3,4):tags[key]=struct.unpack_from(e+('H' if ty==3 else 'I'),data,at+8)[0]
    if 322 in tags:
     w,h,bw,bh=tags[256],tags[257],tags[322],tags[323];across=(w+bw-1)//bw;out=bytearray(w*h*3)
     for y in range(h):
      for x in range(w):
       k=(y//bh)*across+x//bw;at=(k*bw*bh+(y%bh)*bw+x%bw)*3;out[(y*w+x)*3:(y*w+x)*3+3]=actual[at:at+3]
     actual=bytes(out)
    self.assertEqual(actual,pixels(self.path/'core.pnm'),name)
if __name__=='__main__':unittest.main(verbosity=2)
