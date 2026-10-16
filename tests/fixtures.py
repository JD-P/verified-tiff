"""Independent byte-level TIFF fixture writer, not part of the decoder."""
import struct


def packbits_row(row, literal_only=False, noops=False):
    """Independent row encoder with literal and repeated-byte packets."""
    encoded=bytearray();i=0
    while i<len(row):
        if noops: encoded.append(128)
        run=1
        while i+run<len(row) and row[i+run]==row[i] and run<128: run+=1
        if not literal_only and run>=3:
            encoded.extend((257-run,row[i]));i+=run
        else:
            first=i;i+=1
            while i<len(row) and i-first<128:
                if not literal_only and i+2<len(row) and row[i]==row[i+1]==row[i+2]: break
                i+=1
            encoded.append(i-first-1);encoded.extend(row[first:i])
    if noops: encoded.append(128)
    return bytes(encoded)


def lzw_codes(codes, late_change=False):
    """Pack explicit codes, tracking wire widths independently of byte phrases."""
    result=bytearray();acc=0;bits=0;next_code=258;has_previous=False
    for code in codes:
        a,b,c=(512,1024,2048) if late_change else (511,1023,2047)
        width=9 if next_code<a else 10 if next_code<b else 11 if next_code<c else 12
        assert 0<=code<(1<<width)
        acc=(acc<<width)|code;bits+=width
        while bits>=8:
            bits-=8;result.append((acc>>bits)&255)
        acc &= (1<<bits)-1
        if code==256: next_code=258;has_previous=False
        elif code!=257:
            if has_previous: next_code+=1
            has_previous=True
    if bits: result.append((acc<<(8-bits))&255)
    return bytes(result)


def lzw_encode(data, eoi=True):
    """Independent string-to-code encoder, with Clear at dictionary saturation."""
    table={bytes([b]):b for b in range(256)};next_code=258
    codes=[256];word=b''
    for b in data:
        char=bytes([b]);candidate=word+char
        if candidate in table: word=candidate;continue
        codes.append(table[word]);table[candidate]=next_code;next_code+=1
        if next_code==4095:
            codes.append(256);table={bytes([b]):b for b in range(256)};next_code=258
        word=char
    if word: codes.append(table[word])
    if eoi: codes.append(257)
    return lzw_codes(codes)


def canonical_alpha(raw,channels,white,mode):
    """Reference normalization using rational arithmetic and half-up rounding."""
    from fractions import Fraction
    out=bytearray(raw)
    for i in range(0,len(raw),channels):
        a=raw[i+channels-1]
        for c in range(channels-1):
            if mode==1 and a==0: out[i+c]=0;continue
            v=raw[i+c]
            if mode==1: v=min(255,int(Fraction(v*255,a)+Fraction(1,2)))
            out[i+c]=255-v if white else v
    return bytes(out)


def make_tiff(width=7, height=5, channels=1, white=False, le=True,
              tiled=False, before=False, rows=3, pages=1, extra=None,
              bits=8, fill=1, pixel_value=None, compression=1,
              literal_only=False, noops=False, encoded_block=None, alpha=0, stored_value=None):
    order = '<' if le else '>'
    pack = lambda fmt, *values: struct.pack(order + fmt, *values)
    data = bytearray((b'II' if le else b'MM') + pack('HI', 42, 0))
    links, expected = [], []

    def allocate(blob):
        if len(data) % 2: data.append(0)
        pos = len(data)
        data.extend(blob)
        return pos

    for page in range(pages):
        def sample(x, y, c):
            if pixel_value is not None: return pixel_value(x, y, c, page)
            if alpha and c==channels-1: return [0,1,2,127,128,254,255][(x+3*y+page)%7]
            if bits == 1: return 255 if ((x*13 + y*7 + page*3) % 11) < 5 else 0
            return (x*17 + y*31 + c*73 + page*11) % 256
        pixels = bytes(sample(x,y,c)
                       for y in range(height) for x in range(width) for c in range(channels))
        stored=pixels
        if alpha:
            assert channels in (2,4) and bits==8 and alpha in (1,2)
            native=bytearray()
            for y in range(height):
                for x in range(width):
                    opacity=pixels[(y*width+x)*channels+channels-1]
                    for c in range(channels):
                        v=pixels[(y*width+x)*channels+c]
                        if c!=channels-1:
                            if white: v=255-v
                            if alpha==1: v=(v*opacity+127)//255
                        if stored_value is not None: v=stored_value(x,y,c,page)
                        native.append(v)
            stored=bytes(native)
            pixels=canonical_alpha(stored,channels,white,alpha)
        expected.append(pixels)
        bw, bh = (16, 16) if tiled else (width, rows)
        across = (width + bw - 1)//bw
        down = (height + bh - 1)//bh
        blocks = []
        for ty in range(down):
            for tx in range(across):
                block = bytearray()
                for y in range(bh if tiled else min(bh, height-ty*bh)):
                    if bits == 1:
                        assert channels == 1
                        # Deliberately nonzero padding, independent of logical pixels.
                        row = bytearray([0xa5 ^ ((y+tx+ty) & 1)*0xff] * ((bw+7)//8))
                        for x in range(bw):
                            xx, yy = tx*bw+x, ty*bh+y
                            v = pixels[yy*width+xx] if xx<width and yy<height else 255
                            assert v in (0,255)
                            bit = (v == 255) != white
                            mask = 1 << (x%8 if fill == 2 else 7-x%8)
                            row[x//8] = (row[x//8] | mask) if bit else (row[x//8] & ~mask)
                        block.extend(row)
                        continue
                    for x in range(bw):
                        for c in range(channels):
                            xx, yy = tx*bw+x, ty*bh+y
                            v = stored[(yy*width+xx)*channels+c] if xx<width and yy<height else 0
                            block.append(255-v if white and not alpha else v)
                if compression==32773:
                    stride=(bw+7)//8 if bits==1 else bw*channels
                    block=b''.join(packbits_row(block[i:i+stride],literal_only,noops)
                                   for i in range(0,len(block),stride))
                elif compression==5: block=lzw_encode(block)
                if encoded_block is not None: block=encoded_block(bytes(block))
                blocks.append(bytes(block))
        tags = {256:(4,[width]),257:(4,[height]),258:(3,[bits]*channels),
                259:(3,[compression]),262:(3,[0 if white else 1 if channels<=2 else 2]),
                277:(3,[channels]),65000:(2,b'independent fixture\0')}
        if alpha: tags[338]=(3,[alpha])
        if fill != 1: tags[266]=(3,[fill])
        offsets_tag, counts_tag = (324,325) if tiled else (273,279)
        if tiled: tags.update({322:(4,[bw]),323:(4,[bh])})
        else: tags[278]=(4,[rows])
        tags[offsets_tag] = (4,[0]*len(blocks))
        tags[counts_tag] = (4,list(map(len,blocks)))
        if extra:
            for key,value in extra.items():
                if value is None: tags.pop(key,None)
                else: tags[key]=value
        offsets = [allocate(block) for block in blocks] if before else None
        ifd = allocate(bytes(2+12*len(tags)+4))
        links.append(ifd)
        if offsets is None: offsets = [allocate(block) for block in blocks]
        tags[offsets_tag] = (4,offsets)
        data[ifd:ifd+2] = pack('H',len(tags))
        for i,(tag,(kind,values)) in enumerate(sorted(tags.items())):
            blob = values if isinstance(values,bytes) else pack({3:'H',4:'I'}[kind]*len(values),*values)
            count = len(values)
            field = blob.ljust(4,b'\0') if len(blob)<=4 else pack('I',allocate(blob))
            pos=ifd+2+i*12
            data[pos:pos+12]=pack('HHI',tag,kind,count)+field
    data[4:8]=pack('I',links[0])
    for i,ifd in enumerate(links[:-1]):
        n=struct.unpack_from(order+'H',data,ifd)[0]
        data[ifd+2+n*12:ifd+6+n*12]=pack('I',links[i+1])
    return bytes(data),expected


def entry(data,tag):
    order='<' if data[:2]==b'II' else '>'
    ifd=struct.unpack_from(order+'I',data,4)[0]
    n=struct.unpack_from(order+'H',data,ifd)[0]
    for i in range(n):
        off=ifd+2+i*12
        if struct.unpack_from(order+'H',data,off)[0]==tag: return off
    raise KeyError(tag)


def change_inline(data,tag,value,kind=None):
    data=bytearray(data);off=entry(data,tag)
    order='<' if data[:2]==b'II' else '>'
    actual=struct.unpack_from(order+'H',data,off+2)[0]
    if kind is not None: struct.pack_into(order+'H',data,off+2,kind)
    struct.pack_into(order+('H' if actual==3 else 'I'),data,off+8,value)
    return bytes(data)
