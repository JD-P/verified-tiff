"""Independent fixtures and libtiff comparisons for palette/planar/16-bit TIFF."""
import itertools, json, pathlib, struct, subprocess, tempfile, unittest, zlib
from fixtures import packbits_row, lzw_encode, canonical_alpha, change_inline, entry

ROOT=pathlib.Path(__file__).resolve().parents[1]
BIN=ROOT/'target/debug/verified-tiff'
ORACLE=ROOT/'target/native-oracle'

def make_extended(w=17,h=19,ch=3,bits=16,palette=False,separate=False,le=True,
                  tiled=False,compression=1,white=False,alpha=0,before=False,pages=1,deflater=None,strip_rows=3,pad_last=False,transform=None,predictor=1,fill=1,sample_format=None,sample_values=None):
    if sample_format is None:sample_format=3 if bits==64 else 1
    order='<' if le else '>'
    pack=lambda fmt,*args:struct.pack(order+fmt,*args)
    data=bytearray((b'II' if le else b'MM')+pack('HI',42,0));links=[];expected=[];natives=[]
    def alloc(b):
        if len(data)%2:data.append(0)
        pos=len(data);data.extend(b);return pos
    for page in range(pages):
        colors=1<<bits if palette else 0
        cmap=[((c*18731+i*65521+page*257)%65536) for c in range(3) for i in range(colors)]
        # Includes nonzero low bytes, endpoints, and alpha boundary values.
        maximum=(1<<bits)-1
        native=[]
        for y in range(h):
            for x in range(w):
                for c in range(ch):
                    if sample_values is not None:v=sample_values[((y*w+x)*ch+c+page)%len(sample_values)]
                    elif palette:v=(x*7+y*13+page)%colors
                    elif alpha and c==ch-1:v=[0,1,2,127,128,254,255][(x+3*y+page)%7]
                    else:v=([0,maximum,1,maximum//2,maximum-1][(x+y+c)%5] if x<5 else (x*1543+y*8191+c*23003+page*97)&maximum)
                    native.append(v)
        native_bytes=b''.join(v.to_bytes(8,'little') if bits==64 else v.to_bytes(2,'big') if bits==16 else bytes([v]) for v in native)
        natives.append(native_bytes)
        if palette:out=bytes(cmap[c*colors+idx]>>8 for idx in native for c in range(3))
        elif bits==64:out=native_bytes
        elif bits==1:out=bytes(255*(1-v if white else v) for v in native)
        elif alpha:out=canonical_alpha(native_bytes,ch,white,alpha)
        else:out=b''.join((maximum-v if white else v).to_bytes(2 if bits==16 else 1,'big') for v in native)
        expected.append(out)
        bw,bh=(16,16) if tiled else (w,strip_rows)
        blocks=[]
        for plane in range(ch if separate else 1):
            for y0 in range(0,h,bh):
                for x0 in range(0,w,bw):
                    rows=[]
                    for yy in range(bh if tiled or pad_last else min(bh,h-y0)):
                        values=[]
                        for xx in range(bw):
                            for c in ([plane] if separate else range(ch)):
                                v=native[((y0+yy)*w+x0+xx)*ch+c] if x0+xx<w and y0+yy<h else maximum
                                values.append(v)
                        if bits==64 and predictor==2:
                            stride=1 if separate else ch
                            values=[(v-values[i-stride])&maximum if i>=stride else v for i,v in enumerate(values)]
                        if bits<8:
                            row=bytearray((len(values)*bits+7)//8)
                            for i,v in enumerate(values):row[i*bits//8]|=v<<(8-bits-(i*bits)%8)
                        else:row=b''.join(v.to_bytes(bits//8,'little' if le else 'big') for v in values)
                        rows.append(bytes(row))
                    raw=b''.join(rows)
                    encoded=b''.join(packbits_row(r,noops=True) for r in rows) if compression==32773 else lzw_encode(raw) if compression==5 else (deflater or zlib.compress)(raw) if compression in (8,32946) else raw
                    blocks.append(transform(encoded,len(blocks),raw) if transform else encoded)
        ot,ct=(324,325) if tiled else (273,279)
        tags={256:(4,[w]),257:(4,[h]),258:(3,[bits]*ch),259:(3,[compression]),
              262:(3,[3 if palette else 0 if white else 1 if ch<=2 else 2]),277:(3,[ch]),
              284:(3,[2 if separate else 1]),ot:(4,[0]*len(blocks)),ct:(4,list(map(len,blocks))),
              339:(3,[sample_format]*ch),317:(3,[predictor]),266:(3,[fill])}
        if tiled:tags.update({322:(4,[bw]),323:(4,[bh])})
        else:tags[278]=(4,[bh])
        if palette:tags[320]=(3,cmap)
        if alpha:tags[338]=(3,[alpha])
        offsets=[alloc(b) for b in blocks] if before else None
        ifd=alloc(bytes(2+12*len(tags)+4));links.append(ifd)
        if offsets is None:offsets=[alloc(b) for b in blocks]
        tags[ot]=(4,offsets);data[ifd:ifd+2]=pack('H',len(tags))
        for i,(tag,(kind,vs)) in enumerate(sorted(tags.items())):
            blob=pack({3:'H',4:'I'}[kind]*len(vs),*vs)
            field=blob.ljust(4,b'\0') if len(blob)<=4 else pack('I',alloc(blob))
            pos=ifd+2+12*i;data[pos:pos+12]=pack('HHI',tag,kind,len(vs))+field
    data[4:8]=pack('I',links[0])
    for a,b in zip(links,links[1:]):
        count=struct.unpack_from(order+'H',data,a)[0];data[a+2+12*count:a+6+12*count]=pack('I',b)
    return bytes(data),expected,natives

def pixels(path):
    b=path.read_bytes()
    if b.startswith(b'\x93NUMPY'):return b[10+int.from_bytes(b[8:10],'little'):]
    return b.split(b'ENDHDR\n',1)[1] if b.startswith(b'P7') else b.split(b'\n',3)[3]

class Extended(unittest.TestCase):
    def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
    def tearDown(self):self.tmp.cleanup()
    def run_data(self,data,*flags):
        src=self.path/'in.tif';out=self.path/'out.pnm';src.write_bytes(data)
        p=subprocess.run([str(BIN),str(src),str(out),*map(str,flags)],capture_output=True,timeout=10)
        return p,out
    def check(self,options,page=0,pages=1):
        data,expected,native=make_extended(**options,pages=pages)
        p,out=self.run_data(data,'--page',page)
        self.assertEqual(p.returncode,0,p.stderr)
        self.assertEqual(pixels(out),expected[page],options)
        meta=json.loads(p.stdout)
        self.assertEqual(meta['output_bits_per_sample'],16 if options.get('bits',16)==16 else 8)
        self.assertLessEqual(meta['scratch_bytes'],meta['output_bytes'])
        if ORACLE.exists():
            ref=self.path/'native.raw'
            q=subprocess.run([str(ORACLE),str(self.path/'in.tif'),str(ref),str(page)],capture_output=True,timeout=10)
            self.assertEqual(q.returncode,0,q.stderr)
            if options.get('palette'):self.assertEqual(ref.read_bytes(),expected[page])
            else:self.assertEqual(ref.read_bytes(),native[page])
        return data,meta
    def test_palette_matrix(self):
        n=0
        for bits,le,tiled,compression,before,(w,h) in itertools.product([1,4,8],[False,True],[False,True],[1,32773,5],[False,True],[(1,1),(7,5),(17,19),(33,17)]):
            with self.subTest(bits=bits,le=le,tiled=tiled,compression=compression,w=w,h=h):
                self.check(dict(w=w,h=h,ch=1,bits=bits,palette=True,le=le,tiled=tiled,compression=compression,before=before));n+=1
        print(f'palette matrix: {n}; native oracle={ORACLE.exists()}')
    def test_16bit_matrix(self):
        n=0
        for ch,white in [(1,False),(1,True),(3,False)]:
            for le,tiled,separate,compression,before,(w,h) in itertools.product([False,True],[False,True],[False,True],[1,32773,5],[False,True],[(1,1),(7,5),(17,19),(33,17)]):
                with self.subTest(ch=ch,white=white,le=le,tiled=tiled,separate=separate,compression=compression,w=w,h=h):
                    self.check(dict(w=w,h=h,ch=ch,white=white,bits=16,separate=separate,le=le,tiled=tiled,compression=compression,before=before));n+=1
        print(f'16-bit matrix: {n}; native oracle={ORACLE.exists()}')
    def test_separate_8bit_matrix(self):
        n=0
        for ch,white,alpha in [(1,False,0),(1,True,0),(3,False,0),(2,False,1),(2,True,1),(2,False,2),(2,True,2),(4,False,1),(4,False,2)]:
            for le,tiled,compression,(w,h) in itertools.product([False,True],[False,True],[1,32773,5],[(1,1),(7,5),(17,19)]):
                with self.subTest(ch=ch,white=white,alpha=alpha,le=le,tiled=tiled,compression=compression,w=w,h=h):
                    self.check(dict(w=w,h=h,ch=ch,white=white,alpha=alpha,bits=8,separate=True,le=le,tiled=tiled,compression=compression));n+=1
        print(f'8-bit separate matrix: {n}; native oracle={ORACLE.exists()}')
    def test_pages_and_limits(self):
        for options in [dict(ch=1,bits=4,palette=True),dict(ch=3,bits=16,separate=True,compression=5),dict(ch=4,bits=8,separate=True,alpha=1,compression=32773,tiled=True)]:
            for page in range(3):self.check(options,page=page,pages=3)
            data,meta=self.check(options)
            p,_=self.run_data(data,'--work',meta['work_used']);self.assertEqual(p.returncode,0,p.stderr)
            for flag,value in [('--work',meta['work_used']-1),('--max-output',meta['output_bytes']-1)]:
                p,_=self.run_data(data,flag,value);self.assertEqual(p.returncode,2)
    def test_invalid_new_metadata(self):
        for options in [dict(w=1,h=1,ch=1,bits=4,palette=True),dict(w=1,h=1,ch=3,bits=16,separate=True),dict(w=1,h=1,ch=4,bits=8,separate=True,alpha=1)]:
            data,_,_=make_extended(**options)
            for n in range(len(data)):
                p,_=self.run_data(data[:n]);self.assertEqual(p.returncode,2,(options,n))
            for tag,value in [(284,3),(258,12),(339,3)]:
                bad=change_inline(data,tag,value)
                p,_=self.run_data(bad);self.assertEqual(p.returncode,2)
        data,_,_=make_extended(ch=1,bits=4,palette=True)
        for kind,count in [(4,48),(3,47),(3,49),(3,0)]:
            bad=bytearray(data);p=entry(bad,320);bad[p+2:p+8]=struct.pack('<HI',kind,count)
            result,_=self.run_data(bad);self.assertEqual(result.returncode,2)

if __name__=='__main__':unittest.main(verbosity=2)
