"""Independent VP8L writer for known-pixel tests, based on the public bitstream spec.
Canonical codes are assigned here from lengths, independently of the Rust decoder.
"""
import collections,pathlib,struct,subprocess
from fixtures import make_tiff
ROOT=pathlib.Path(__file__).resolve().parents[1]
ORDER=[17,18,0,1,2,3,4,5,16,6,7,8,9,10,11,12,13,14,15]
class Bits:
    def __init__(self):self.bits=[]
    def put(self,value,n):
        assert 0<=value<1<<n
        self.bits.extend((value>>i)&1 for i in range(n))
    def code(self,code,n):self.bits.extend((code>>i)&1 for i in reversed(range(n)))
    def data(self):
        a=self.bits+[0]*(-len(self.bits)%8)
        return bytes(sum(a[i+j]<<j for j in range(8)) for i in range(0,len(a),8))
def canonical(lengths):
    counts=collections.Counter(lengths);code=0;first={}
    for n in range(1,16):code=(code+(counts[n-1] if n>1 else 0))*2;first[n]=code
    out={}
    for symbol,n in enumerate(lengths):
        if n:out[symbol]=(first[n],n);first[n]+=1
    if len(out)==1:out={next(iter(out)):(0,0)}
    return out

def table(bits,alphabet,symbols,normal=False):
    symbols=sorted(set(symbols)) or [0]
    if len(symbols)<=2 and max(symbols)<256 and not normal:
        bits.put(1,1);bits.put(len(symbols)-1,1);bits.put(int(symbols[0]>1),1);bits.put(symbols[0],8 if symbols[0]>1 else 1)
        if len(symbols)==2:bits.put(symbols[1],8)
        return {s:(i,1 if len(symbols)==2 else 0) for i,s in enumerate(symbols)}
    lengths=[0]*alphabet
    if len(symbols)==1:lengths[symbols[0]]=1
    else:
        k=len(symbols).bit_length()-1;short=(1<<(k+1))-len(symbols)
        for i,s in enumerate(symbols):lengths[s]=k if i<short else k+1
    bits.put(0,1);bits.put(15,4)
    for s in ORDER:bits.put(4 if s<16 else 0,3)
    bits.put(0,1)
    for n in lengths:bits.code(n,4)
    return canonical(lengths)
def prefix(value):
    for c in range(40):
        n=0 if c<4 else (c-2)//2;base=c+1 if c<4 else (2+c%2)*(1<<n)+1
        if base<=value<base+(1<<n):return c,value-base,n
    raise ValueError(value)
def cache_hash(p,b):return ((int.from_bytes(bytes([p[2],p[1],p[0],p[3]]),'little')*0x1e35a7bd)&0xffffffff)>>(32-b)
def riff(payload):
    chunk=b'VP8L'+struct.pack('<I',len(payload))+payload+b'\0'*(len(payload)%2)
    return b'RIFF'+struct.pack('<I',len(chunk)+4)+b'WEBP'+chunk

def encode_tokens(w,h,tokens,subtract=False,cache=0,alpha_hint=True,normal=False):
    bits=Bits()
    bits.put(int(subtract),1)
    if subtract:bits.put(2,2);bits.put(0,1)
    bits.put(int(cache>0),1)
    if cache:bits.put(cache,4)
    bits.put(0,1)
    symbols=[[],[],[],[],[]]
    for t in tokens:
        if t[0]=='L':
            r,g,b,a=t[1];symbols[0].append(g);symbols[1].append(r);symbols[2].append(b);symbols[3].append(a)
        elif t[0]=='C':symbols[0].append(256+prefix(t[1])[0]);symbols[4].append(prefix(t[2])[0])
        else:symbols[0].append(280+t[1])
    codes=[table(bits,n,syms,normal) for n,syms in zip([280+(1<<cache if cache else 0),256,256,256,40],symbols)]
    def sym(group,s):bits.code(*codes[group][s])
    for t in tokens:
        if t[0]=='L':
            r,g,b,a=t[1];sym(0,g);sym(1,r);sym(2,b);sym(3,a)
        elif t[0]=='C':
            lc,lv,ln=prefix(t[1]);dc,dv,dn=prefix(t[2]);sym(0,256+lc);bits.put(lv,ln);sym(4,dc);bits.put(dv,dn)
        else:sym(0,280+t[1])
    header=(w-1)|((h-1)<<14)|(int(alpha_hint)<<28)
    return riff(b'\x2f'+struct.pack('<I',header)+bits.data())

def encode(raw,w,ch=4,subtract=False,cache=0,copies=False,normal=False):
    pixels=[tuple(raw[i:i+ch])+( (255,) if ch==3 else ()) for i in range(0,len(raw),ch)];h=len(pixels)//w
    src=[((r-g)%256,g,(b-g)%256,a) if subtract else (r,g,b,a) for r,g,b,a in pixels]
    colors=[(0,0,0,0)]*(1<<cache) if cache else [];tokens=[];i=0
    while i<len(src):
        n=0;dist=0
        if copies and i:
            for d in dict.fromkeys([1,min(i,w),min(i,3),min(i,17)]):
                k=0
                while k<4096 and i+k<len(src) and src[i+k]==src[i+k-d]:k+=1
                if k>n:n=k;dist=d
        if n>=3:
            # Codes 1 and 2 mean previous row and previous pixel; the linear
            # distance region is also exercised by other distances.
            code=1 if dist==w else 2 if dist==1 else dist+120
            tokens.append(('C',n,code));emitted=src[i:i+n];i+=n
        else:
            p=src[i];key=cache_hash(p,cache) if cache else None
            tokens.append(('K',key) if cache and colors[key]==p else ('L',p));emitted=[p];i+=1
        if cache:
            for p in emitted:colors[cache_hash(p,cache)]=p
    return encode_tokens(w,h,tokens,subtract,cache,any(p[3]!=255 for p in pixels),normal)

def make_webp(width=7,height=5,channels=4,alpha=2,subtract=False,cache=0,copies=False,normal=False,**kw):
    bw=16 if kw.get('tiled') else width
    return make_tiff(width=width,height=height,channels=channels,alpha=alpha,compression=50001,
        encoded_block=lambda raw:encode(raw,bw,channels,subtract,cache,copies,normal),**kw)
def oracle(blob,tmp):
    src=tmp/'segment.webp';dst=tmp/'segment.rgba';src.write_bytes(blob)
    r=subprocess.run([str(ROOT/'target/webp-oracle'),str(src),str(dst)],capture_output=True,timeout=10)
    assert r.returncode==0,r.stderr
    return dst.read_bytes()
def wrap(blob,w,h,ch=4,alpha=2,**kw):
    return make_tiff(width=w,height=h,channels=ch,alpha=alpha,rows=h,compression=50001,encoded_block=lambda _:blob,**kw)[0]
