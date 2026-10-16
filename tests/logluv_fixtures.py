"""Independent SGILog32 encoder and exact rational LogLuv16 reference."""
import struct
from fractions import Fraction


def native(words):
    result=bytearray()
    for word in words:
        l=word>>16
        # Independent rational conversion to libtiff's signed Luv48 container.
        u=int(Fraction(((word>>8)&255)*2+1,2)*Fraction(32768,410))
        v=int(Fraction((word&255)*2+1,2)*Fraction(32768,410))
        result.extend(struct.pack('<HHH',l,u,v))
    return bytes(result)


def plane_encode(row,mode='mixed'):
    out=bytearray();i=0
    while i<len(row):
        if mode=='noops':out.extend(b'\0\0')
        run=1
        while i+run<len(row) and row[i+run]==row[i] and run<129:run+=1
        if mode!='literal' and run>=2:out.extend([run+126,row[i]]);i+=run
        else:
            start=i;i+=1
            while i<len(row) and i-start<127:
                if mode!='literal' and i+1<len(row) and row[i]==row[i+1]:break
                i+=1
            out.append(i-start);out.extend(row[start:i])
    return bytes(out)


def encode(words,width,mode='mixed'):
    result=bytearray()
    for start in range(0,len(words),width):
        row=words[start:start+width]
        for shift in [24,16,8,0]:result.extend(plane_encode(bytes((v>>shift)&255 for v in row),mode))
    if mode=='noops':result.extend(b'\0\0')
    return bytes(result)


def make(w=17,h=19,le=True,tiled=False,rows=7,mode='mixed',pattern='varied',pages=1,words=None,transform=None,overrides=None):
    order='<' if le else '>';pack=lambda fmt,*vs:struct.pack(order+fmt,*vs)
    data=bytearray((b'II' if le else b'MM')+pack('HI',42,0));links=[];expected=[];raw_pages=[]
    def alloc(b):
        if len(data)%2:data.append(0)
        at=len(data);data.extend(b);return at
    for page in range(pages):
        values=[(words[i%len(words)] if words else
                 (([0,1,0x7fff,0x8000,0x8001,0xffff][(i+page)%6]<<16)|(((i*37+page*19)&255)<<8)|((255-i*29)&255)) if pattern=='varied' else
                 [0,0xffffffff,0x400056c2,0x8000ff00][(i//130+page)%4]) for i in range(w*h)]
        expected.append(native(values));raw_pages.append(values)
        bw,bh=(16,16) if tiled else (w,rows);blocks=[]
        for y in range(0,h,bh):
            for x in range(0,w,bw):
                count=bh if tiled else min(bh,h-y)
                block=[values[(y+dy)*w+x+dx] if y+dy<h and x+dx<w else 0x8123ffff for dy in range(count) for dx in range(bw)]
                encoded=encode(block,bw,mode)
                blocks.append(transform(encoded,len(blocks)) if transform else encoded)
        ot,ct=(324,325) if tiled else (273,279)
        tags={256:(4,[w]),257:(4,[h]),258:(3,[16]*3),259:(3,[34676]),262:(3,[32845]),
              277:(3,[3]),284:(3,[1]),339:(3,[2]*3),ot:(4,[0]*len(blocks)),ct:(4,list(map(len,blocks)))}
        if tiled:tags.update({322:(4,[bw]),323:(4,[bh])})
        else:tags[278]=(4,[bh])
        if overrides:tags.update(overrides)
        ifd=alloc(bytes(2+12*len(tags)+4));links.append(ifd)
        offsets=[alloc(b) for b in blocks];tags[ot]=(4,offsets)
        data[ifd:ifd+2]=pack('H',len(tags))
        for i,(tag,(kind,vs)) in enumerate(sorted(tags.items())):
            blob=pack({3:'H',4:'I'}[kind]*len(vs),*vs)
            field=blob.ljust(4,b'\0') if len(blob)<=4 else pack('I',alloc(blob))
            data[ifd+2+12*i:ifd+14+12*i]=pack('HHI',tag,kind,len(vs))+field
    data[4:8]=pack('I',links[0])
    for a,b in zip(links,links[1:]):
        n=struct.unpack_from(order+'H',data,a)[0];data[a+2+12*n:a+6+12*n]=pack('I',b)
    return bytes(data),expected,raw_pages
