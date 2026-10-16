"""Deflate conformance, resource limits, and TIFF differential regression tests.
Python's zlib and libtiff are independent test oracles, never runtime codecs.
"""
import itertools,json,pathlib,struct,subprocess,tempfile,unittest,zlib,random
from extended import make_extended,pixels,ROOT,BIN,ORACLE
from fixtures import canonical_alpha,change_inline

class Bits:
    def __init__(self):self.bits=[]
    def put(self,v,n):self.bits.extend((v>>i)&1 for i in range(n))
    def code(self,v,n):self.bits.extend((v>>i)&1 for i in reversed(range(n)))
    def finish(self):
        b=bytearray((len(self.bits)+7)//8)
        for i,v in enumerate(self.bits):b[i//8]|=v<<(i%8)
        return bytes(b)

def fixed_symbol(b,c):
    if c<=143:b.code(48+c,8)
    elif c<=255:b.code(400+c-144,9)
    elif c<=279:b.code(c-256,7)
    else:b.code(192+c-280,8)

def wrapped(payload,raw=b'',header=b'\x78\x01'):
    return header+payload+struct.pack('>I',zlib.adler32(raw))

def fixed_stream(raw,distance=None,length_code=285,extra=0):
    b=Bits();b.put(1,1);b.put(1,2)
    for v in raw:fixed_symbol(b,v)
    if distance is not None:
        fixed_symbol(b,length_code)
        if length_code==284:b.put(extra,5)
        code,nb,d=distance;b.code(code,5);b.put(d,nb)
    fixed_symbol(b,256);return b

def literal_dynamic(raw):
    """Complete literal tree {0,EOB}; empty distance tree; explicit lengths."""
    assert not any(raw)
    b=Bits();b.put(1,1);b.put(2,2);b.put(0,5);b.put(0,5);b.put(14,4)
    order=[16,17,18,0,8,7,9,6,10,5,11,4,12,3,13,2,14,1]
    for i in order:b.put(1 if i in (0,1) else 0,3)
    for i in range(258):b.code(1 if i in (0,256) else 0,1)
    for _ in raw:b.code(0,1)
    b.code(1,1)
    return wrapped(b.finish(),raw)

def tiny_tiff(stream,w,h=1,rows=None,compression=8):
    """One-strip Gray8 TIFF independent of the general fixture writer."""
    tags=[(256,4,w),(257,4,h),(258,3,8),(259,3,compression),(262,3,1),
          (273,4,0),(277,3,1),(278,4,rows if rows is not None else h),(279,4,len(stream))]
    offset=8+2+12*len(tags)+4
    tags[5]=(273,4,offset)
    b=bytearray(b'II'+struct.pack('<HIH',42,8,len(tags)))
    for tag,kind,value in tags:b+=struct.pack('<HHI',tag,kind,1)+struct.pack('<I',value)
    return bytes(b+bytes(4)+stream)

def compressor(mode):
    if mode=='stored':return lambda raw:zlib.compress(raw,level=0)
    if mode=='fixed':
        def fixed(raw):
            c=zlib.compressobj(level=6,strategy=zlib.Z_FIXED);return c.compress(raw)+c.flush()
        return fixed
    return zlib.compress

class Deflate(unittest.TestCase):
    def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
    def tearDown(self):self.tmp.cleanup()
    def run_data(self,data,*flags):
        src=self.path/'in.tif';out=self.path/'ours.pnm';src.write_bytes(data)
        p=subprocess.run([str(BIN),str(src),str(out),*map(str,flags)],capture_output=True,timeout=15)
        return p,out
    def check(self,opts,mode='default'):
        data,expected,native=make_extended(**opts,deflater=compressor(mode))
        p,out=self.run_data(data)
        self.assertEqual(p.returncode,0,(opts,mode,p.stderr))
        self.assertEqual(pixels(out),expected[0],(opts,mode))
        meta=json.loads(p.stdout)
        if ORACLE.exists():
            mono=opts.get('bits')==1 and not opts.get('palette')
            oracle=ROOT/'target/libtiff-oracle' if mono else ORACLE
            ref=self.path/'reference.raw'
            q=subprocess.run([str(oracle),str(self.path/'in.tif'),str(ref),'0'],capture_output=True,timeout=15)
            self.assertEqual(q.returncode,0,q.stderr)
            self.assertEqual(pixels(ref)[::3] if mono else ref.read_bytes(),expected[0] if mono or opts.get('palette') else native[0])
        return data,meta
    def test_format_matrix(self):
        profiles=[dict(ch=ch,white=white,alpha=alpha,bits=8) for ch,white,alpha in
                  [(1,False,0),(1,True,0),(3,False,0),(2,False,1),(2,True,1),(2,False,2),(2,True,2),(4,False,1),(4,False,2)]]
        profiles += [dict(ch=ch,white=white,bits=bits) for bits in (1,16) for ch,white in ([(1,False),(1,True)] if bits==1 else [(1,False),(1,True),(3,False)])]
        profiles += [dict(ch=1,palette=True,bits=bits) for bits in (1,4,8)]
        n=0
        for profile,le,tiled,separate,compression,mode in itertools.product(profiles,[False,True],[False,True],[False,True],[8,32946],['stored','fixed','default']):
            options=dict(profile,w=17,h=19,le=le,tiled=tiled,separate=separate,compression=compression)
            with self.subTest(options=options,mode=mode):self.check(options,mode);n+=1
        print(f'Deflate matrix: {n}; live oracle={ORACLE.exists()}')
    def test_padded_final_strip(self):
        for ch,bits,alpha in [(1,1,0),(1,8,0),(3,8,0),(4,8,1),(3,16,0)]:
            for separate,le in itertools.product([False,True],repeat=2):
                data,meta=self.check(dict(w=17,h=8,ch=ch,bits=bits,alpha=alpha,strip_rows=3,pad_last=True,separate=separate,le=le,compression=8))
                p,_=self.run_data(data,'--work',meta['work_used']);self.assertEqual(p.returncode,0,p.stderr)
                p,_=self.run_data(data,'--work',meta['work_used']-1);self.assertEqual(p.returncode,2)
        original=ROOT/'tests/upstream/deflate-last-strip-extra-data.tiff'
        p,out=self.run_data(original.read_bytes());self.assertEqual(p.returncode,0,p.stderr)
        ref=self.path/'ref.raw';q=subprocess.run([str(ORACLE),str(original),str(ref),'0'],capture_output=True)
        self.assertEqual(q.returncode,0,q.stderr);self.assertEqual(pixels(out),ref.read_bytes())
        corrupt=bytearray(original.read_bytes());corrupt[-1]^=1
        p,_=self.run_data(corrupt);self.assertEqual(p.returncode,2)
    def test_block_kinds_history_and_window(self):
        cases=[(b'',zlib.compress(b'')),(bytes(400),literal_dynamic(bytes(400)))]
        # Overlapping copy, maximum distance, history wrap, and multiple block boundaries.
        raw=b'A'*259;b=fixed_stream(b'A',(0,0,0));cases.append((raw,wrapped(b.finish(),raw)))
        rng=random.Random(1951);seed=bytes(rng.randrange(256) for _ in range(32768));raw=seed+seed[:258]
        b=fixed_stream(seed,(29,13,8191));cases.append((raw,wrapped(b.finish(),raw)))
        raw=seed*3;c=zlib.compressobj();stream=c.compress(raw[:12000])+c.flush(zlib.Z_SYNC_FLUSH)+c.compress(raw[12000:70000])+c.flush(zlib.Z_FULL_FLUSH)+c.compress(raw[70000:])+c.flush();cases.append((raw,stream))
        for mode in ['stored','fixed','default']:
            raw=bytes(range(251))*800;cases.append((raw,compressor(mode)(raw)))
        for raw,stream in cases:
            self.assertEqual(zlib.decompress(stream),raw)
            # TIFF dimensions must be positive; empty stream is an underflow case.
            p,out=self.run_data(tiny_tiff(stream,max(1,len(raw))), '--max-dimension',len(raw)+1)
            self.assertEqual(p.returncode,0 if raw else 2,p.stderr)
            if raw:self.assertEqual(pixels(out),raw)
    def test_malformed_streams(self):
        valid=zlib.compress(bytes(range(32)))
        bad=[valid[:-1],valid+bytes(1),valid+valid,valid[:2]+b'\x07'+valid[3:],valid[:-1]+bytes([valid[-1]^1]),b'\x78\x00'+valid[2:],b'\x79\x18'+valid[2:],b'\x88\x1c'+valid[2:],b'\x78\x20'+valid[2:]]
        # Reserved block type; invalid stored complement; reserved literal/distance codes;
        # distance before history, distance outside the advertised zlib window.
        b=Bits();b.put(1,1);b.put(3,2);bad.append(wrapped(b.finish()))
        bad.append(wrapped(b'\x01\x20\x00\x00\x00'+bytes(32)))
        for code in (286,287):
            b=Bits();b.put(1,1);b.put(1,2);fixed_symbol(b,code);bad.append(wrapped(b.finish()))
        for d in (30,31):bad.append(wrapped(fixed_stream(b'A',(d,0,0)).finish()))
        bad.append(wrapped(fixed_stream(b'',(0,0,0)).finish()))
        raw=b'A'*300;b=fixed_stream(raw,(16,7,43)) # distance 300, exceeds 256-byte declared window
        bad.append(wrapped(b.finish(),raw+b'A'*258,b'\x08\x1d'))
        # Dynamic code-length alphabet oversubscription/incompleteness and illegal repeats.
        for lens,tail in [([1,1,1,0],None),([2,2,0,0],None),([1,0,1,0],('previous',)),([0,0,1,1],('overflow',))]:
            b=Bits();b.put(1,1);b.put(2,2);b.put(0,5);b.put(0,5);b.put(0,4)
            for v in lens:b.put(v,3)
            if tail==('previous',):b.code(0,1);b.put(0,2)
            elif tail==('overflow',):
                for _ in range(2):b.code(1,1);b.put(127,7)
            bad.append(wrapped(b.finish()))
        # Complete code-length alphabet 0..15. Independently malformed data trees.
        good_lit=[0]*257;good_lit[0]=good_lit[256]=1
        for mode in ['missing-eob','lit-over','lit-incomplete','dist-over','dist-incomplete']:
            lit=good_lit.copy();dist=[0]
            if mode=='missing-eob':lit[256]=0;lit[1]=1
            elif mode=='lit-over':lit[1]=1
            elif mode=='lit-incomplete':lit[0]=lit[256]=2
            elif mode=='dist-over':dist=[1,1,1]
            else:dist=[2,2]
            b=Bits();b.put(1,1);b.put(2,2);b.put(0,5);b.put(len(dist)-1,5);b.put(15,4)
            for k in [16,17,18,0,8,7,9,6,10,5,11,4,12,3,13,2,14,1,15]:b.put(4 if k<16 else 0,3)
            for v in lit+dist:b.code(v,4)
            bad.append(wrapped(b.finish()))
        b=Bits();b.put(1,1);b.put(1,2)
        for v in bytes(32):fixed_symbol(b,v)
        bad.append(wrapped(b.finish(),bytes(32))) # no EOB
        for stream in bad:
            p,_=self.run_data(tiny_tiff(stream,32));self.assertEqual(p.returncode,2,stream.hex())
        for n in range(len(valid)):
            p,_=self.run_data(tiny_tiff(valid[:n],32));self.assertEqual(p.returncode,2,n)
    def test_expansion_caps_and_metadata(self):
        for raw,w,h,rows,ok in [(bytes(32),16,2,2,True),(bytes(33),16,2,2,False),(bytes(31),16,2,2,False),(bytes(48),16,2,3,False)]:
            p,_=self.run_data(tiny_tiff(zlib.compress(raw),w,h,rows));self.assertEqual(p.returncode,0 if ok else 2,p.stderr)
        bomb=zlib.compress(bytes(2_000_000))
        p,_=self.run_data(tiny_tiff(bomb,1));self.assertEqual(p.returncode,2)
        data,_,_=make_extended(w=17,h=19,ch=1,bits=8,tiled=True,compression=8)
        for option in ['fill','predictor']:
            bad,_,_=make_extended(w=17,h=19,ch=1,bits=8,compression=8,**{option:2})
            p,_=self.run_data(bad);self.assertEqual(p.returncode,2)
        p,_=self.run_data(data,'--max-output',255);self.assertEqual(p.returncode,2)
        # Deflate alias, unsupported predictor, and shared budget boundary.
        data,meta=self.check(dict(w=17,h=19,ch=3,bits=16,separate=True,compression=32946))
        p,_=self.run_data(data,'--work',0);self.assertEqual(p.returncode,2)
    def test_multipage(self):
        data,expected,_=make_extended(ch=3,bits=16,separate=True,compression=8,pages=3)
        for page in range(3):
            p,out=self.run_data(data,'--page',page);self.assertEqual(p.returncode,0,p.stderr);self.assertEqual(pixels(out),expected[page])

def write_seeds():
    root=ROOT/'tests/fixtures'
    cases=[('deflate-stored',dict(ch=1,bits=8),'stored'),
           ('deflate-fixed',dict(ch=3,bits=8),'fixed'),
           ('deflate-bilevel',dict(ch=1,bits=1,white=True,tiled=True),'default'),
           ('deflate-palette',dict(ch=1,bits=4,palette=True,tiled=True),'default'),
           ('deflate-planar16',dict(ch=3,bits=16,separate=True,pad_last=True),'default'),
           ('deflate-alpha',dict(ch=4,bits=8,alpha=1,separate=True),'default')]
    for name,opts,mode in cases:
        data,expected,_=make_extended(w=17,h=8,compression=8,**opts,deflater=compressor(mode))
        (root/(name+'.tif')).write_bytes(data);(root/(name+'.pixels')).write_bytes(expected[0])
    raw=bytes(200)
    (root/'deflate-dynamic.tif').write_bytes(tiny_tiff(literal_dynamic(raw),20,10))
    (root/'deflate-dynamic.pixels').write_bytes(raw)
    stream=bytes([120,1,1,1,0,254,255,65,0,66,0,66])
    (root/'deflate-witness.tif').write_bytes(tiny_tiff(stream,1))
    (root/'deflate-witness.pixels').write_bytes(b'A')

if __name__=='__main__':
    import sys
    if sys.argv[1:]==['--write-seeds']:write_seeds()
    else:unittest.main(verbosity=2)
