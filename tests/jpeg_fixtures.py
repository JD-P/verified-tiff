"""Independent TIFF container writer around the pinned test-only JPEG encoder."""
import pathlib,struct,subprocess,tempfile
from fractions import Fraction
from math import lcm
ROOT=pathlib.Path(__file__).resolve().parents[1]
CJPEG=ROOT/'.tools/jpeg/bin/cjpeg'

def marker(code,payload):return b'\xff'+bytes([code])+struct.pack('>H',len(payload)+2)+payload

def parse_jpeg(data):
    assert data[:2]==b'\xff\xd8';at=2;headers=[]
    while at<len(data):
        assert data[at]==255;code=data[at+1];n=int.from_bytes(data[at+2:at+4],'big');payload=data[at+4:at+2+n];headers.append((code,payload));at+=n+2
        if code==218:return headers,data[at:-2]
    raise AssertionError('missing scan')

def tables(headers):
    q={};h={}
    for code,payload in headers:
        i=0
        if code==219:
            while i<len(payload):assert payload[i]<4;q[payload[i]]=payload[i+1:i+65];i+=65
        if code==196:
            while i<len(payload):n=sum(payload[i+1:i+17]);h[payload[i]]=payload[i+1:i+17+n];i+=17+n
    return q,h

def encode(w,h,ch,hs,vs,quality=85,restart=0,rgb=False,optimize=False,seed=0):
    raw=bytes((x*13+y*29+c*71+seed*17+((x//5+y//3)%2)*93)%256 for y in range(h) for x in range(w) for c in range(ch))
    pnm=(f'P{5 if ch==1 else 6}\n{w} {h}\n255\n').encode()+raw
    args=[str(CJPEG),'-quality',str(quality),'-sample',f'{hs}x{vs},1x1,1x1']
    if ch==1:args+=['-grayscale']
    if rgb:args+=['-rgb']
    if restart:args+=['-restart',f'{restart}B']
    if optimize:args+=['-optimize']
    p=subprocess.run(args,input=pnm,capture_output=True,timeout=15)
    if p.returncode:raise AssertionError((args,p.stderr))
    return p.stdout

def make_jpeg_tiff(w=17,h=19,ch=3,hs=2,vs=2,le=True,tiled=False,mode='shared',quality=85,restart=0,photo=None,limited=False,holes=False,optimize=False,strip_rows=None,seed=0,references=None):
    photo=photo if photo is not None else (1 if ch==1 else 6)
    if photo!=6:hs=vs=1
    order='<' if le else '>';pack=lambda fmt,*v:struct.pack(order+fmt,*v)
    data=bytearray((b'II' if le else b'MM')+pack('HI',42,0))
    def alloc(blob):
        if len(data)%2:data.append(0)
        p=len(data);data.extend(blob);return p
    bw,bh=(16,16) if tiled else (w,strip_rows or 8*vs)
    blocks=[];parsed=[]
    for y in range(0,h,bh):
        for x in range(0,w,bw):
            rows=bh if tiled else min(bh,h-y)
            blob=encode(bw,rows,ch,hs,vs,quality,restart,photo==2,optimize,seed+x+y)
            parsed.append(parse_jpeg(blob));blocks.append(blob)
            if references is not None:references.append((x,y,bw,rows,blob))
    tags={256:(4,[w]),257:(4,[h]),258:(3,[8]*ch),259:(3,[6 if mode.startswith('old') else 7]),262:(3,[photo]),277:(3,[ch]),284:(3,[1])}
    if photo==6:
        tags[530]=(3,[hs,vs]);tags[532]=(5,[(16,1),(235,1),(128,1),(240,1),(128,1),(240,1)] if limited else [(0,1),(255,1),(128,1),(255,1),(128,1),(255,1)])
    if tiled:tags[322]=(4,[bw]);tags[323]=(4,[bh]);okey,bkey=324,325
    else:tags[278]=(4,[bh]);okey,bkey=273,279
    first=parsed[0][0];q,huff=tables(first)
    if mode=='shared':
        assert all(tables(h)==(q,huff) for h,_ in parsed)
        shared=b'\xff\xd8'+b''.join(marker(c,p) for c,p in first if c in (219,196))+b'\xff\xd9';tags[347]=(7,shared)
        blocks=[b'\xff\xd8'+b''.join(marker(c,p) for c,p in header if c not in (219,196))+entropy+b'\xff\xd9' for header,entropy in parsed]
    elif mode.startswith('old'):
        assert all(tables(h)==(q,huff) for h,_ in parsed)
        tags[512]=(3,[1]);tags[515]=(3,[restart])
        sof=next(p for c,p in first if c in (192,193));sos=next(p for c,p in first if c==218)
        qt=[sof[8+3*i] for i in range(ch)];dc=[sos[2+2*i]//16 for i in range(ch)];ac=[16+sos[2+2*i]%16 for i in range(ch)]
        tags[519]=(4,[alloc(q[i]) for i in qt]);tags[520]=(4,[alloc(huff[i]) for i in dc]);tags[521]=(4,[alloc(huff[i]) for i in ac])
        if mode=='old-header':
            assert not tiled
            header=[]
            for c,p in first:
                if c==218:continue
                if c in (192,193):p=p[:1]+struct.pack('>H',h)+p[3:]
                header.append(marker(c,p))
            blob=b'\xff\xd8'+b''.join(header);tags[513]=(4,[alloc(blob)]);tags[514]=(4,[len(blob)])
            blocks=[(marker(218,next(p for c,p in head if c==218)) if i==0 else b'')+entropy+(b'\xff\xd9' if i==len(parsed)-1 else b'') for i,(head,entropy) in enumerate(parsed)]
        else:blocks=[entropy for _,entropy in parsed]
    else:assert mode=='self'
    tags[okey]=(4,[alloc(b) for b in blocks]);tags[bkey]=(4,list(map(len,blocks)))
    entries=[]
    for key,(kind,values) in sorted(tags.items()):
        if kind in (1,2,7):payload=bytes(values);n=len(payload)
        elif kind==5:payload=b''.join(pack('II',*v) for v in values);n=len(values)
        else:payload=pack(('H' if kind==3 else 'I')*len(values),*values);n=len(values)
        slot=payload.ljust(4,b'\0') if len(payload)<=4 else pack('I',alloc(payload))
        entries.append(pack('HHI',key,kind,n)+slot)
    if holes:entries=entries[::2]+[bytes(12),bytes(12)]+entries[1::2]
    off=alloc(pack('H',len(entries))+b''.join(entries)+bytes(4));data[4:8]=pack('I',off)
    return bytes(data)

def reference_rgb(components,photo,rbw):
    if photo!=6:return components
    rbw=list(map(Fraction,rbw));yr=255/(rbw[1]-rbw[0]);br=127/(rbw[3]-rbw[2]);rr=127/(rbw[5]-rbw[4]);matrix=[]
    # Derive RGB from exact TIFF luma coefficients (299,587,114)/1000.
    for cb,cr in [(Fraction(0),Fraction(1402,1000)),(-Fraction(202008,587000),-Fraction(419198,587000)),(Fraction(1772,1000),Fraction(0))]:
        co=[yr,cb*br,cr*rr,-yr*rbw[0]-cb*br*rbw[2]-cr*rr*rbw[4]];den=lcm(*(c.denominator for c in co));matrix.append(([int(c*den) for c in co],den))
    out=bytearray()
    for i in range(0,len(components),3):
        y,b,c=components[i:i+3]
        for (a,d,e,z),den in matrix:out.append(max(0,min(255,(a*y+d*b+e*c+z+den//2)//den)))
    return bytes(out)

def oracle(src,out):
    p=subprocess.run([str(ROOT/'target/jpeg-tiff-oracle'),str(src),str(out)],capture_output=True,timeout=30)
    if p.returncode:raise AssertionError((src,p.stderr))
    parts=p.stdout.decode().split();return reference_rgb(out.read_bytes(),int(parts[3]),parts[6:])


def generated_oracle(options,references,tmp):
    """Decode the writer's original JPEGs independently, before TIFF wrapping.
    This avoids libtiff OJPEG's forced RGB-to-YCbCr tag reinterpretation and
    its raw-component adapter's differing vertical sampling behavior.
    """
    w=options.get('w',17);h=options.get('h',19);ch=options.get('ch',3)
    photo=options.get('photo',1 if ch==1 else 6);out=bytearray(w*h*ch)
    rbw=[16,235,128,240,128,240] if options.get('limited') else [0,255,128,255,128,255]
    for x,y,bw,bh,blob in references:
        src=tmp/'reference.jpg';dest=tmp/'reference.components';src.write_bytes(blob)
        args=[str(ROOT/'target/jpeg-oracle'),str(src),str(dest)]
        if photo==6:args.append('ycbcr')
        subprocess.run(args,check=True,capture_output=True,timeout=15)
        data=reference_rgb(dest.read_bytes(),photo,rbw)
        assert len(data)==bw*bh*ch
        for r in range(min(bh,h-y)):
            n=min(bw,w-x)*ch;start=((y+r)*w+x)*ch
            out[start:start+n]=data[r*bw*ch:r*bw*ch+n]
    return bytes(out)
