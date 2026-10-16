"""Known-pixel, independent libwebp/libtiff, and malformed VP8L regressions."""
import itertools,json,pathlib,struct,subprocess,tempfile,unittest
from fixtures import canonical_alpha,change_inline,entry
from extended import pixels
from webp_fixtures import *
BIN=ROOT/'target/debug/verified-tiff'
class WebpTests(unittest.TestCase):
    def setUp(self):self.temp=tempfile.TemporaryDirectory();self.tmp=pathlib.Path(self.temp.name)
    def tearDown(self):self.temp.cleanup()
    def decode(self,blob,expect=None,page=0):
        src=self.tmp/'input.tif';out=self.tmp/'output.pam';src.write_bytes(blob)
        r=subprocess.run([str(BIN),str(src),str(out),'--page',str(page),'--work','20000000000'],capture_output=True,timeout=10)
        if expect is None:return r
        self.assertEqual(r.returncode,0,r.stderr);self.assertEqual(pixels(out),expect)
        return json.loads(r.stdout)
    def segment(self,blob,w,h,expect,ch=4,alpha=2):
        self.assertEqual(oracle(blob,self.tmp),expect)
        native=expect if ch==4 else bytes(v for i,v in enumerate(expect) if i%4!=3)
        if alpha:native=canonical_alpha(native,ch,False,alpha)
        return self.decode(wrap(blob,w,h,ch,alpha),native)
    def test_format_matrix(self):
        count=0
        for le,tiled,before,subtract,cache,(ch,alpha) in itertools.product((False,True),(False,True),(False,True),(False,True),(0,1,7,11),((3,0),(4,1),(4,2))):
            data,expected=make_webp(width=19 if tiled else 7,height=17 if tiled else 5,channels=ch,alpha=alpha,le=le,tiled=tiled,before=before,subtract=subtract,cache=cache)
            meta=self.decode(data,expected[0]);self.assertEqual(meta['compression'],50001);self.assertEqual(meta['channels'],ch)
            src=self.tmp/'input.tif';ref=self.tmp/'native.raw'
            q=subprocess.run([str(ROOT/'target/native-oracle'),str(src),str(ref),'0'],capture_output=True,timeout=10)
            self.assertEqual(q.returncode,0,q.stderr);native=ref.read_bytes()
            if alpha:native=canonical_alpha(native,ch,False,alpha)
            self.assertEqual(native,expected[0]);count+=1
        self.assertEqual(count,192);print(f'WebP profile matrix: {count} cases; libtiff/native exact')
    def test_original_opaque_alpha(self):
        src=ROOT/'tests/upstream/webp_lossless_rgba_alpha_fully_opaque.tif';data=src.read_bytes()
        off=struct.unpack_from('<I',data,entry(data,273)+8)[0];n=struct.unpack_from('<I',data,entry(data,279)+8)[0]
        expected=oracle(data[off:off+n],self.tmp);self.assertEqual(set(expected[3::4]),{255})
        self.assertEqual((int.from_bytes(data[off+21:off+25],'little')>>28)&1,0)
        meta=self.decode(data,expected);self.assertEqual(meta['channels'],4);self.assertEqual(meta['source_alpha'],'unassociated')
    def test_cache_and_copy_history(self):
        for cb in range(1,12):
            raw=bytes([0,0,0,0,3,8,77,91,11,31,251,255])*7
            b=encode(raw,7,cache=cb,copies=True,subtract=cb%2==0)
            self.segment(b,7,3,raw)
            b=encode_tokens(2,1,[('K',0),('K',0)],cache=cb)
            self.segment(b,2,1,b'\0'*8)
        for n in [1,2,3,4,5,6,7,8,9,12,13,16,17,24,25,32,33,48,49,64,65,96,97,128,129,192,193,256,257,384,385,512,513,768,769,1024,1025,1536,1537,2048,2049,3072,3073,4096]:
            p=(77,33,251,127);b=encode_tokens(n+1,1,[('L',p),('C',n,2)])
            self.segment(b,n+1,1,bytes(p)*(n+1))
        # A long overlapping copy followed by a cache hit proves copied colors
        # populate the cache, and green reversal happens after copying/cache use.
        p=(200,100,250,17);cb=11;k=cache_hash(p,cb)
        b=encode_tokens(42,1,[('L',p),('C',40,2),('K',k)],subtract=True,cache=cb)
        self.segment(b,42,1,bytes([44,100,94,17])*42)
    def test_long_distance_and_normal_single_leaf(self):
        for d in [1,2,3,8,17,119,120,121,4096,65536,262144]:
            first=(101,19,200,17);rest=(33,85,7,255)
            tokens=[('L',first)];left=d-1
            if left:tokens.append(('L',rest));left-=1
            while left:n=min(left,4096);tokens.append(('C',n,2));left-=n
            tokens.append(('C',1,d+120))
            # Width divides the image and stays within VP8L's 14-bit dimensions.
            total=d+1;w=1;h=total
            if h>16384:
                pad=(-total)%1024;tokens.extend([('L',rest)]*pad);w=1024;h=(total+pad)//w
            else:pad=0
            b=encode_tokens(w,h,tokens,normal=True)
            self.segment(b,w,h,bytes(first)+bytes(rest)*(d-1)+bytes(first)+bytes(rest)*pad)
        b=encode_tokens(1,1,[('L',(0,1,2,255))],normal=True)
        self.segment(b,1,1,bytes([0,1,2,255]))
    def test_all_plane_distance_codes(self):
        offsets=[(0, 1), (1, 0), (1, 1), (-1, 1), (0, 2), (2, 0), (1, 2), (-1, 2), (2, 1), (-2, 1), (2, 2), (-2, 2), (0, 3), (3, 0), (1, 3), (-1, 3), (3, 1), (-3, 1), (2, 3), (-2, 3), (3, 2), (-3, 2), (0, 4), (4, 0), (1, 4), (-1, 4), (4, 1), (-4, 1), (3, 3), (-3, 3), (2, 4), (-2, 4), (4, 2), (-4, 2), (0, 5), (3, 4), (-3, 4), (4, 3), (-4, 3), (5, 0), (1, 5), (-1, 5), (5, 1), (-5, 1), (2, 5), (-2, 5), (5, 2), (-5, 2), (4, 4), (-4, 4), (3, 5), (-3, 5), (5, 3), (-5, 3), (0, 6), (6, 0), (1, 6), (-1, 6), (6, 1), (-6, 1), (2, 6), (-2, 6), (6, 2), (-6, 2), (4, 5), (-4, 5), (5, 4), (-5, 4), (3, 6), (-3, 6), (6, 3), (-6, 3), (0, 7), (7, 0), (1, 7), (-1, 7), (5, 5), (-5, 5), (7, 1), (-7, 1), (4, 6), (-4, 6), (6, 4), (-6, 4), (2, 7), (-2, 7), (7, 2), (-7, 2), (3, 7), (-3, 7), (7, 3), (-7, 3), (5, 6), (-5, 6), (6, 5), (-6, 5), (8, 0), (4, 7), (-4, 7), (7, 4), (-7, 4), (8, 1), (8, 2), (6, 6), (-6, 6), (8, 3), (5, 7), (-5, 7), (7, 5), (-7, 5), (8, 4), (6, 7), (-6, 7), (7, 6), (-7, 6), (8, 5), (7, 7), (-7, 7), (8, 6), (8, 7)]
        for w in [1,17]:
            for code,(dx,dy) in enumerate(offsets,1):
                d=max(1,dx+dy*w);before=max(d,128);count=before+7;pad=(-count)%w;count+=pad
                history=[((i*13)%256,(i*29)%256,(i*57)%256,255) for i in range(before)]
                tokens=[('L',p) for p in history]+[('C',7,code)]
                for _ in range(7):history.append(history[-d])
                for _ in range(pad):history.append((0,0,0,255));tokens.append(('L',(0,0,0,255)))
                b=encode_tokens(w,count//w,tokens)
                self.segment(b,w,count//w,bytes(c for p in history for c in p))
    def test_multi_page_and_metadata_rejections(self):
        data,expected=make_webp(pages=3,le=False,subtract=True,cache=4)
        for page in range(3):self.decode(data,expected[page],page)
        data,_=make_webp()
        for tag,v in [(259,50002),(262,6),(277,2),(338,0),(278,1)]:self.assertNotEqual(self.decode(change_inline(data,tag,v)).returncode,0)
        for extra in [{317:(3,[2])},{284:(3,[2])},{266:(3,[2])},{274:(3,[2])},{339:(3,[3,3,3,3])},{258:(3,[16,16,16,16])}]:
            data,_=make_webp(extra=extra);self.assertNotEqual(self.decode(data).returncode,0)
    def test_truncation_dimensions_container_and_stream_rejections(self):
        p=(7,19,251,177);good=encode_tokens(2,1,[('L',p),('C',1,2)],cache=4)
        for n in range(len(good)):self.assertNotEqual(self.decode(wrap(good[:n],2,1)).returncode,0,n)
        bad=[]
        for index in [0,8,12,20]:
            b=bytearray(good);b[index]^=1;bad.append(bytes(b))
        b=bytearray(good);b[21]^=1;bad.append(bytes(b))
        b=bytearray(good);b[24]|=0x20;bad.append(bytes(b))
        bad.extend([good+b'\0',good[:-1]+b'\xff',encode_tokens(1,1,[('C',1,2)]),encode_tokens(2,1,[('L',p),('C',2,2)])])
        # A valid RIFF with an unused whole payload byte is outside the exact-length profile.
        payload=good[20:20+int.from_bytes(good[16:20],'little')];bad.append(riff(payload+b'\0'))
        for b in bad:self.assertNotEqual(self.decode(wrap(b,2,1)).returncode,0)
        for typ in [0,1,3]:
            b=bytearray(good);b[25]=(b[25]&~7)|1|(typ<<1);self.assertNotEqual(self.decode(wrap(bytes(b),2,1)).returncode,0)
    def test_code_length_repetitions_and_token_limit(self):
        for limited in [False,True]:
            bits=Bits();bits.put(0,3);bits.put(0,1);bits.put(15,4)
            cl=[0]*19
            for sym,n in [(0,2),(8,2),(16,2),(17,3),(18,3)]:cl[sym]=n
            for sym in ORDER:bits.put(cl[sym],3)
            codes=canonical(cl);lengths=[8]*128+[0]+[8]*128+[0]*23
            tokens=[]
            for side in range(2):
                left=128
                while left>=3:
                    n=min(left,6);tokens.append((16,n-3,2));left-=n
                tokens.extend((8,0,0) for _ in range(left))
                if side==0:tokens.append((0,0,0))
            if not limited:tokens.extend([(17,0,3),(18,9,7)])
            bits.put(int(limited),1)
            if limited:bits.put(2,3);bits.put(len(tokens)-2,6)
            for sym,value,n in tokens:bits.code(*codes[sym]);bits.put(value,n)
            for n,syms in [(256,[9]),(256,[17]),(256,[255]),(40,[0])]:table(bits,n,syms)
            green=canonical(lengths)
            for value in [0,127,129,255]:bits.code(*green[value])
            header=3
            b=riff(b'\x2f'+struct.pack('<I',header)+bits.data())
            self.segment(b,4,1,b''.join(bytes([9,g,17,255]) for g in [0,127,129,255]))
        # A complete skewed tree exercises the maximum supported length 15.
        bits=Bits();bits.put(0,3);bits.put(0,1);bits.put(15,4)
        for symbol in ORDER:bits.put(4 if symbol<16 else 0,3)
        bits.put(0,1);lens=list(range(1,15))+[15,15]+[0]*264
        for n in lens:bits.code(n,4)
        for n,symbols in [(256,[19]),(256,[23]),(256,[255]),(40,[0])]:table(bits,n,symbols)
        codes=canonical(lens)
        for g in [0,14,15,9]:bits.code(*codes[g])
        b=riff(b'\x2f'+struct.pack('<I',3)+bits.data())
        self.segment(b,4,1,b''.join(bytes([19,g,23,255]) for g in [0,14,15,9]))
    def test_canonical_huffman_rejections(self):
        # One legal simple table is replaced by an out-of-range distance symbol.
        bits=Bits();bits.put(0,3)
        for s in [0,0,0,255,255]:table(bits,256,[s])
        b=riff(b'\x2f'+b'\0'*4+bits.data());self.assertNotEqual(self.decode(wrap(b,1,1)).returncode,0)
        for lengths in [[0]*19,[1]*19,[2]+[0]*18]:
            bits=Bits();bits.put(0,3);bits.put(0,1);bits.put(15,4)
            for s in ORDER:bits.put(lengths[s],3)
            bits.put(0,1);bits.put(0,16)
            b=riff(b'\x2f'+b'\0'*4+bits.data());self.assertNotEqual(self.decode(wrap(b,1,1)).returncode,0)
        # Invalid cache widths 0 and 12..15 and unsupported entropy meta-image.
        for cb in [0,12,13,14,15]:
            bits=Bits();bits.put(0,1);bits.put(1,1);bits.put(cb,4);bits.put(0,8)
            b=riff(b'\x2f'+b'\0'*4+bits.data());self.assertNotEqual(self.decode(wrap(b,1,1)).returncode,0)
        bits=Bits();bits.put(0,2);bits.put(1,1);bits.put(0,8)
        b=riff(b'\x2f'+b'\0'*4+bits.data());self.assertNotEqual(self.decode(wrap(b,1,1)).returncode,0)
if __name__=='__main__':unittest.main(verbosity=2)
