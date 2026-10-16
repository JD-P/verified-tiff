"""Independent sample bytes, bounded live memory, actual public read contracts.
The same C consumer is built against the adapter and pinned libtiff.
"""
import itertools,os,pathlib,random,struct,subprocess,sys,tempfile,unittest,zlib
from coverage import assemble,scene,ROOT,OUT
sys.path.insert(0,str(ROOT/'tests'))
from fixtures import lzw_encode,packbits_row

def encode(rows,codec):
    raw=b''.join(rows)
    return raw if codec==1 else b''.join(packbits_row(row) for row in rows) if codec==32773 else lzw_encode(raw) if codec==5 else zlib.compress(raw)

def image(rows,w,bits=8,fmt=1,ch=1,codec=1,predictor=1,le=True,big=False,photo=1,fill=1):
    encoded=encode(rows,codec)
    if fill==2:encoded=bytes(int(f'{v:08b}'[::-1],2) for v in encoded)
    return assemble([[(256,4,[w]),(257,4,[len(rows)]),(258,3,[bits]*ch),(259,3,[codec]),(262,3,[photo]),(266,3,[fill]),(273,16 if big else 4,('SEG',[encoded])),(277,3,[ch]),(278,4,[len(rows)]),(279,16 if big else 4,[len(encoded)]),(284,3,[1]),(317,3,[predictor]),(339,3,[fmt]*ch)]],le=le,big=big)[0]

def packed(values,bits):
    n=(len(values)*bits+7)//8
    value=0
    for v in values:value=value<<bits|v
    padding=n*8-len(values)*bits
    return ((value<<padding)|((1<<padding)-1)).to_bytes(n,'big')

def fp_predict(row,width,ch):
    # Independent TIFF Predictor 3 significance-plane shuffle and differencing.
    words=[row[i:i+width][::-1] for i in range(0,len(row),width)]
    a=bytearray(b''.join(bytes(word[p] for word in words) for p in range(width)))
    for i in range(len(a)-1,ch-1,-1):a[i]=(a[i]-a[i-ch])&255
    return bytes(a)

class ReaderFeatures(unittest.TestCase):
    def setUp(self):self.tmp=tempfile.TemporaryDirectory();self.p=pathlib.Path(self.tmp.name)
    def tearDown(self):self.tmp.cleanup()
    def run_reader(self,mode,data,*args,oracle=True):
        f=self.p/'in.tif';f.write_bytes(data);out=self.p/'ours';ref=self.p/'ref'
        subprocess.run([str(OUT/'reader-features'),mode,str(f),str(out),*map(str,args)],check=True)
        actual=out.read_bytes()
        if oracle:
            subprocess.run([str(OUT/'reader-features-libtiff'),mode,str(f),str(ref),*map(str,args)],check=True)
            self.assertEqual(actual,ref.read_bytes(),(mode,args))
        return actual
    def check_native(self,data,expected,oracle=True):
        f=self.p/'in.tif';f.write_bytes(data)
        for mode in ['scan','native']:
            out=self.p/'ours';r=subprocess.run([str(OUT/'dump'),str(f),mode,str(out)],capture_output=True)
            self.assertEqual(r.returncode,0,(mode,r.stderr));self.assertEqual(out.read_bytes(),expected)
            if oracle:
                ref=self.p/'ref';r=subprocess.run([str(OUT/'dump-libtiff'),str(f),mode,str(ref)],capture_output=True)
                self.assertEqual(r.returncode,0,(mode,r.stderr));self.assertEqual(ref.read_bytes(),expected)
    def test_packed_rgba_tables_crops_and_fillorder(self):
        for bits,photo,codec,fill in itertools.product([1,2,4,8],[0,1,3],[1,5,8],[1,2]):
            if bits==8 and photo!=3:continue
            mask=(1<<bits)-1;per=8//bits;w=256*per-3;h=3
            values=[[(byte>>(8-bits-j*bits))&mask for byte in range(256) for j in range(per)][:w] for _ in range(h)]
            rows=[packed(v,bits) for v in values];encoded=encode(rows,codec)
            if fill==2:encoded=bytes(int(f'{v:08b}'[::-1],2) for v in encoded)
            cmap=[(i*317+c*17381)&65535 for c in range(3) for i in range(mask+1)]
            tags=[(256,4,[w]),(257,4,[h]),(258,3,[bits]),(259,3,[codec]),(262,3,[photo]),(266,3,[fill]),(273,4,('SEG',[encoded])),(277,3,[1]),(278,4,[h]),(279,4,[len(encoded)])]
            if photo==3:tags.append((320,3,cmap))
            data=assemble([tags])[0]
            def pixel(v):
                if photo==3:return bytes([cmap[c*(mask+1)+v]>>8 for c in range(3)]+[255])
                gray=(mask-v if photo==0 else v)*255//mask
                return bytes([gray,gray,gray,255])
            for x,cw in [(0,w),(1,89),(3,87),(7,81)]:
                for orientation in [1,2,3,4]:
                    xs=list(range(x,x+cw));ys=list(range(2))
                    if orientation in (2,3):xs.reverse()
                    if orientation in (3,4):ys.reverse()
                    expected=b''.join(pixel(values[y][xx]) for y in ys for xx in xs)
                    # 4.7.1 applies its column offset to a byte pointer even
                    # for packed samples; nonzero crops use independent pixels.
                    self.assertEqual(self.run_reader('crop',data,cw,2,orientation,x,0,oracle=x==0),expected)

    def test_packed_and_unusual_integer_widths(self):
        for bits,codec,le in itertools.product([3,5,6,7,9,10,11,12,13,14,15,17,20,24,31,40,48,56,63], [1,32773,5,8],[False,True]):
            w,ch,h=7,3,3;rows=[];expected=[]
            for y in range(h):
                values=[((i*104729+y*7331)^(1<<bits-1))&((1<<bits)-1) for i in range(w*ch)]
                if bits==24:
                    rows.append(b''.join(v.to_bytes(3,'little' if le else 'big') for v in values));expected.append(b''.join(v.to_bytes(3,sys.byteorder) for v in values))
                else:rows.append(packed(values,bits));expected.append(rows[-1])
            self.check_native(image(rows,w,bits,1,ch,codec,le=le,big=bits%2==0),b''.join(expected))
    def test_float_and_complex_transport(self):
        for bits,fmt,codec,le in itertools.product([16,24,32,64,128],[3,5,6],[1,32773,5,8],[False,True]):
            if fmt==3 and bits==128 or fmt==5 and bits not in [32,64,128] or fmt==6 and bits not in [64,128]:continue
            component=bits//16 if fmt in [5,6] else bits//8;w,ch=7,2;rows=[];expected=[]
            for y in range(3):
                values=[(i*0x9e3779b9+y*0x10101)&((1<<(8*component))-1) for i in range(w*ch*(2 if fmt in [5,6] else 1))]
                values[:4]=[0,1<<(component*8-1),(1<<(component*8))-1,(1<<(component*8-1))-1]
                rows.append(b''.join(v.to_bytes(component,'little' if le else 'big') for v in values));expected.append(b''.join(v.to_bytes(component,sys.byteorder) for v in values))
            self.check_native(image(rows,w,bits,fmt,ch,codec,le=le,big=True),b''.join(expected))
    def test_half_float_prediction(self):
        for bits,codec,le,ch in itertools.product([16,24,32,64],[5,8],[False,True],[1,3]):
            width=bits//8;w=11;native=[];rows=[]
            for y in range(4):
                values=[(i*7331+y*3359)&((1<<bits)-1) for i in range(w*ch)]
                row=b''.join(v.to_bytes(width,'little') for v in values);native.append(row);rows.append(fp_predict(row,width,ch))
            self.check_native(image(rows,w,bits,3,ch,codec,3,le),b''.join(native))
    def test_cropped_rgba_and_untouched_extra_rows(self):
        for tiled,photo,orientation in itertools.product([False,True],[1,2],range(1,9)):
            data,*_=scene(w=29,h=21,ch=1 if photo==1 else 3,tiled=tiled,photo=photo,orientation=orientation)
            for w,h,requested in [(29,21,1),(9,7,1),(13,19,4),(29,30,1),(9,30,4)]:self.run_reader('crop',data,w,h,requested)
    def test_reader_extensions_and_color_helpers(self):
        data,*_=scene(w=7,h=5)
        self.run_reader('api',data)
        self.run_reader('colors',data)
        self.run_reader('logcolor',data)
        self.run_reader('client',data)
    def test_streaming_large_strips(self):
        for codec,random_data in itertools.product([1,32773,5,8],[False,True]):
            rng=random.Random(941);w,h=1024,384
            rows=[rng.randbytes(w) if random_data else bytes([y%256])*w for y in range(h)]
            data=image(rows,w,codec=codec)
            self.assertEqual(self.run_reader('stream',data),b''.join(rows),(codec,random_data))
            self.check_native(data,b''.join(rows))
    def test_large_color_crops(self):
        for photo,ch,codec in itertools.product([2,5,8],[0],[1,8]):
            ch=4 if photo==5 else 3;w,h=1024,384
            rows=[bytes((x*17+y*23+c*37)%256 for x in range(w) for c in range(ch)) for y in range(h)]
            data=image(rows,w,ch=ch,photo=photo,codec=codec)
            self.run_reader('cropbounded',data,9,7,1,3,190)
    def test_full_color_lzw_and_deflate_staging(self):
        for photo,bits,codec,le in itertools.product([5,8],[8,16],[5,8],[False,True]):
            w,h=257,97;ch=4 if photo==5 else 3;mask=(1<<bits)-1;rows=[];expected=bytearray()
            for y in range(h):
                values=[(x*1013+y*3137+c*9109)&mask for x in range(w) for c in range(ch)]
                rows.append(b''.join(v.to_bytes(bits//8,'little' if le else 'big') for v in values))
                if photo==5:
                    for x in range(w):
                        v=values[x*ch:(x+1)*ch]
                        if bits==16:v=[(q+128)//257 for q in v]
                        expected.extend([(255-v[3])*(255-q)//255 for q in v[:3]]+[255])
            data=image(rows,w,bits,ch=ch,codec=codec,le=le,photo=photo)
            actual=self.run_reader('crop',data,w,h,1,oracle=photo==8 or bits==8)
            if photo==5:self.assertEqual(actual,expected)

    def test_full_deflate_rgba_rows_prediction_and_endian(self):
        # One decoded strip >64 KiB triggers optional complete staging. The
        # Mirrored crops exercise the bounded path as well.
        for bits,ch,predictor,le,codec in itertools.product([8,16],[1,3],[1,2],[False,True],[5,8]):
            w,h=257,131;mask=(1<<bits)-1;rows=[];expected=bytearray()
            for y in range(h):
                values=[(x*7331+y*3359+c*1777)&mask for x in range(w) for c in range(ch)]
                for x in range(w):
                    pixel=[values[x*ch+c] for c in range(ch)]
                    pixel=[(v>>8) if bits==16 and ch==1 else (v*255+mask//2)//mask for v in pixel]
                    expected.extend((pixel*3 if ch==1 else pixel)+[255])
                encoded=[(v-(values[i-ch] if predictor==2 and i>=ch else 0))&mask for i,v in enumerate(values)]
                rows.append(b''.join(v.to_bytes(bits//8,'little' if le else 'big') for v in encoded))
            data=image(rows,w,bits,ch=ch,codec=codec,predictor=predictor,le=le,photo=2 if ch==3 else 1)
            self.assertEqual(self.run_reader('crop',data,w,h,1),expected)
            for req in [2,3,4]:
                cropped=b''.join(expected[((19+(96-yy if req in (3,4) else yy))*w+17+(102-xx if req in (2,3) else xx))*4:((19+(96-yy if req in (3,4) else yy))*w+17+(102-xx if req in (2,3) else xx))*4+4] for yy in range(97) for xx in range(103))
                # 4.7.1 applies a byte rather than a word-scaled col_offset to
                # these 16-bit crops. Use independent expected pixels there.
                self.assertEqual(self.run_reader('crop',data,103,97,req,17,19,oracle=bits==8),cropped)
    def test_rgba_extra_channel_selection_and_alpha_rounding(self):
        for bits,photo,ch,extra in itertools.product([8,16],[0,1,2],[4,6],[(0,1),(0,2),(1,2)]):
            # Exact independent presentation expectation, including the existing
            # grayscale alpha convention. Unspecified channels are ignored.
            base=3 if photo==2 else 1
            if len(extra)>ch-base:continue
            data,*_=scene(w=19,h=7,ch=ch,bits=bits,photo=photo,extra=[(338,3,list(extra))])
            mask=(1<<bits)-1;expected=bytearray();aidx=ch-len(extra)+next(i for i,v in enumerate(extra) if v)
            for y in range(7):
                for x in range(19):
                    values=[(x*1013+y*3137+c*9109+(mask if (x+y+c)%3==0 else 0))&mask for c in range(ch)]
                    cv=lambda v:(v>>8) if bits==16 and base==1 else (v*255+mask//2)//mask
                    pixel=[cv(mask-values[0] if photo==0 else values[c if base==3 else 0]) for c in range(3)]
                    a=cv(values[aidx]);kind=extra[aidx-(ch-len(extra))]
                    if base==3 and kind==2:pixel=[(v*a+127)//255 for v in pixel]
                    expected.extend(pixel+[a])
            self.assertEqual(self.run_reader('crop',data,19,7,1,oracle=False),expected)
    def test_full_rgba_falls_back_under_cumulative_limit(self):
        w,h=512,192
        rows=[bytes((x*17+y*23+c*37)%256 for x in range(w) for c in range(3)) for y in range(h)]
        data=image(rows,w,ch=3,photo=2,codec=8)
        self.assertEqual(self.run_reader('croptight',data,w,h,1),self.run_reader('crop',data,w,h,1))
    def test_late_rgba_segment_failure_is_atomic(self):
        w,h=257,131;rows=[bytes((x*17+y*23)%256 for x in range(w*3)) for y in range(h)]
        for codec in [5,8]:
            good=encode(rows,codec);bad=good+b'junk' if codec==5 else good[:-1]+bytes([good[-1]^1])
            tags=[(256,4,[w]),(257,4,[h*2]),(258,3,[8]*3),(259,3,[codec]),(262,3,[2]),(273,4,('SEG',[good,bad])),(277,3,[3]),(278,4,[h]),(279,4,[len(good),len(bad)])]
            self.run_reader('rejectrgba',assemble([tags])[0],oracle=False)
    def test_streamed_predictors_and_fillorder(self):
        for bits,codec,le,fill in itertools.product([16,24,32,64],[5,8],[False,True],[1,2]):
            width=bits//8;w,ch,h=193,2,96;native=[];rows=[]
            for y in range(h):
                row=b''.join(((i*7331+y*3359)&((1<<bits)-1)).to_bytes(width,'little') for i in range(w*ch))
                native.append(row);rows.append(fp_predict(row,width,ch))
            data=image(rows,w,bits,3,ch,codec,3,le,fill=fill)
            self.assertEqual(self.run_reader('stream',data),b''.join(native))
    def test_custom_directories(self):
        for le,big in itertools.product([False,True],[False,True]):
            tags=scene()[3];offset=(16 if big else 8)+(8 if big else 2)+(20 if big else 12)*len(tags)+(8 if big else 4)
            custom=[(65000,4,[123456]),(65001,3,[2,500,65535]),(65002,2,b'custom metadata\0')]
            data=assemble([tags,custom],le=le,big=big)[0]
            self.run_reader('custom',data,offset)
    def test_stream_failure_is_atomic(self):
        for codec in [32773,5,8]:
            w,h=1024,192;rows=[bytes([y%256])*w for y in range(h)];good=encode(rows,codec)
            bad=[good[:-3],good+b'junk']
            if codec==8:bad.append(good[:-1]+bytes([good[-1]^1]))
            tags=scene(w=w,h=h,ch=1,photo=1)[3]
            for data in bad:
                fields=[(key,ty,[codec] if key==259 else [h] if key==278 else ('SEG',[data]) if key==273 else [len(data)] if key==279 else values) for key,ty,values in tags]
                self.run_reader('reject',assemble([fields])[0],oracle=False)
    def test_regions(self):
        for photo,ch in [(1,1),(2,3),(5,4),(8,3)]:
            for tiled,orientation in itertools.product([False,True],[1,2,3,4]):
                data,*_=scene(w=29,h=21,ch=ch,tiled=tiled,photo=photo,orientation=orientation,codec=8)
                for req in [1,2,3,4,5,6,7,8]:
                    self.run_reader('crop',data,9,7,req,3,2)
    def test_lazy_large_tables(self):
        n=200000
        data=assemble([[(256,4,[1]),(257,4,[n]),(258,3,[8]),(259,3,[1]),(262,3,[1]),(273,4,('SEG',[b'I']*n)),(277,3,[1]),(278,4,[1]),(279,4,[1]*n)]],dedup=True)[0]
        self.assertEqual(self.run_reader('lazy',data,oracle=False),b'I')

if __name__=='__main__':unittest.main(verbosity=2)
