"""Old libtiff LZW: independent string dictionary and LSB wire references."""
import collections, hashlib, itertools, json, pathlib, struct, subprocess, tempfile, unittest
from fixtures import make_tiff, lzw_codes
from extended import make_extended, pixels, ROOT, BIN, ORACLE


def legacy_codes(codes, early=False):
    """Explicit-code packer, independent of phrase construction."""
    result=bytearray();acc=0;bits=0;size=258;previous=False
    for code in codes:
        a,b,c=(511,1023,2047) if early else (512,1024,2048)
        width=9 if size<a else 10 if size<b else 11 if size<c else 12
        assert 0<=code<1<<width
        acc |= code<<bits;bits+=width
        while bits>=8:
            result.append(acc&255);acc >>= 8;bits-=8
        if code==256:size=258;previous=False
        elif code!=257:
            if previous:size=min(4096,size+1)
            previous=True
    if bits:result.append(acc)
    return bytes(result)


def legacy_encode(raw,eoi=True):
    table={bytes([b]):b for b in range(256)};next_code=258;word=b'';codes=[256]
    for value in raw:
        char=bytes([value]);candidate=word+char
        if candidate in table:word=candidate;continue
        codes.append(table[word])
        if next_code<4096:table[candidate]=next_code;next_code+=1
        word=char
    if word:codes.append(table[word])
    if eoi:codes.append(257)
    return legacy_codes(codes)


def reference_stream(encoded):
    """Read bit strings and grow byte strings, never prefix-table entries."""
    wire=''.join(f'{v:08b}'[::-1] for v in encoded)
    table=[bytes([i]) for i in range(256)]+[b'',b'']
    previous=None;pos=0;decoded=bytearray();widths=collections.Counter();clears=0;frozen=0;eoi=False
    while True:
        width=9 if len(table)<512 else 10 if len(table)<1024 else 11 if len(table)<2048 else 12
        if pos+width>len(wire):break
        code=int(wire[pos:pos+width][::-1],2);pos+=width;widths[width]+=1
        if code==256:
            table=[bytes([i]) for i in range(256)]+[b'',b''];previous=None;clears+=1
        elif code==257:eoi=True;break
        else:
            assert code<=len(table) and (code<len(table) or previous is not None)
            phrase=table[code] if code<len(table) else previous+previous[:1]
            assert phrase
            decoded.extend(phrase)
            if previous is not None:
                if len(table)<4096:table.append(previous+phrase[:1])
                else:frozen+=1
            previous=phrase
    return bytes(decoded),dict(complete_eoi=eoi,consumed_bits=pos,remaining_bits=len(wire)-pos,
        next_code_width=width,dictionary_entries=len(table),clears=clears,frozen_steps=frozen,widths=dict(widths))


class LegacyLzw(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
    def tearDown(self):self.tmp.cleanup()
    def run_data(self,data,*flags):
        src=self.path/'in.tif';out=self.path/'out.pnm';src.write_bytes(data)
        result=subprocess.run([str(BIN),str(src),str(out),*map(str,flags)],capture_output=True,timeout=30)
        return result,out
    def check(self,data,expected):
        result,out=self.run_data(data);self.assertEqual(result.returncode,0,result.stderr)
        self.assertEqual(pixels(out),expected)
        return json.loads(result.stdout)
    def oracle(self,expected,bilevel=False):
        out=self.path/'reference.raw';oracle=ROOT/'target/libtiff-oracle' if bilevel else ORACLE
        q=subprocess.run([str(oracle),str(self.path/'in.tif'),str(out),'0'],capture_output=True,timeout=30)
        self.assertEqual(q.returncode,0,q.stderr)
        self.assertEqual(pixels(out)[::3] if bilevel else out.read_bytes(),expected)
    def test_format_matrix(self):
        formats=[dict(bits=1,ch=1,white=v) for v in [False,True]]
        formats += [dict(bits=8,ch=c) for c in [1,3]]
        formats += [dict(bits=8,ch=c,alpha=a) for c,a in [(2,1),(4,2)]]
        formats += [dict(bits=16,ch=c) for c in [1,3]]
        formats += [dict(bits=b,ch=1,palette=True) for b in [1,4,8]]
        formats += [dict(bits=8,ch=3,separate=True),dict(bits=8,ch=4,separate=True,alpha=1)]
        formats += [dict(bits=64,ch=c,predictor=p) for c,p in [(1,1),(1,2),(3,2)]]
        cases=0
        for options,le,tiled,eoi in itertools.product(formats,[False,True],[False,True],[False,True]):
            with self.subTest(options=options,le=le,tiled=tiled,eoi=eoi):
                data,expected,native=make_extended(w=17,h=19,le=le,tiled=tiled,compression=5,
                    transform=lambda encoded,j,raw:legacy_encode(raw,eoi),**options)
                self.check(data,expected[0])
                bilevel=options['bits']==1 and not options.get('palette')
                self.oracle(expected[0] if bilevel or options.get('palette') else native[0],bilevel)
                cases+=1
        self.assertEqual(cases,128)
        print(f'Legacy LZW profile matrix: {cases} cases; independent pixels and live libtiff exact')
    def test_widths_padding_clear_and_full_table(self):
        widths=set();tails=set();cases=0
        for n,clears,eoi in itertools.product([*range(1,9),*range(254,262),*range(766,774),*range(1790,1798),3839,3840,4600],[1,2],[False,True]):
            raw=bytes(i%256 for i in range(n));codes=[256]*clears+list(raw)+([257] if eoi else [])
            encoded=legacy_codes(codes);reference,trace=reference_stream(encoded)
            self.assertEqual(reference,raw);self.assertEqual(trace['complete_eoi'],eoi)
            widths.add(trace['next_code_width']);tails.add(trace['remaining_bits'])
            # Pad bits sit at the high end of the final byte in old-style LZW.
            for fill in [0,((1<<trace['remaining_bits'])-1)<<(8-trace['remaining_bits'])]:
                wire=encoded[:-1]+bytes([encoded[-1]|fill])
                data,_=make_tiff(n,1,rows=1,compression=5,encoded_block=lambda _,e=wire:e)
                self.check(data,raw);cases+=1
            if n in [3839,3840,4600]:self.oracle(raw)
        self.assertEqual(widths,{9,10,11,12});self.assertEqual(tails,set(range(8)))
        for codes,raw in [([256,65,258,259,260,257],b'A'*10),
                          ([256,65,66,258,260,257],b'ABABABA'),
                          ([256,65,258,256,66,258,257],b'AAABBB')]:
            data,_=make_tiff(len(raw),1,rows=1,compression=5,encoded_block=lambda _,c=codes:legacy_codes(c))
            self.check(data,raw);self.oracle(raw);cases+=1
        # Existing dictionary references remain readable after saturation.
        codes=[256]+[i%256 for i in range(3839)]+[258,4095,0,256,65,258,257]
        wire=legacy_codes(codes);raw,trace=reference_stream(wire)
        self.assertEqual(trace['frozen_steps'],3);self.assertEqual(trace['clears'],2)
        data,_=make_tiff(len(raw),1,rows=1,compression=5,encoded_block=lambda _:wire)
        self.check(data,raw);self.oracle(raw);cases+=1
        print(f'Legacy width/padding/dictionary cases: {cases}')
    def test_mixed_segments(self):
        cases=0
        for le,tiled,separate,first in itertools.product([False,True],repeat=4):
            data,expected,_=make_extended(w=17,h=19,bits=16,ch=3,le=le,tiled=tiled,
                separate=separate,compression=5,
                transform=lambda encoded,j,raw:legacy_encode(raw) if (j%2==first) else encoded)
            self.check(data,expected[0]);cases+=1
        self.assertEqual(cases,16)
        # Legacy wire bit order does not enable unsupported TIFF FillOrder 2.
        for le,tiled,fill in itertools.product([False,True],[False,True],[1,2]):
            data,expected,_=make_extended(w=17,h=19,ch=1,bits=1,le=le,tiled=tiled,fill=fill,
                compression=5,transform=lambda encoded,j,raw:legacy_encode(raw))
            if fill==1:self.check(data,expected[0])
            else:self.assertEqual(self.run_data(data)[0].returncode,2)
    def test_reject_malformed_and_hybrid_streams(self):
        for n in [1,7,8,255,767,1791,3840]:
            raw=bytes(i%256 for i in range(n));encoded=legacy_codes([256,*raw])
            bad=[encoded[:-1],encoded+b'\0',encoded+b'\xff',legacy_codes([*raw]),
                legacy_codes([256,*raw,256]),legacy_codes([256,*raw,0]),
                legacy_codes([256,*raw,257])+b'\0',legacy_codes([256,42,259])]
            if n>=255:bad.append(legacy_codes([256,*raw,257],early=True))
            for wire in bad:
                data,_=make_tiff(n,1,rows=1,compression=5,encoded_block=lambda _,e=wire:e)
                self.assertEqual(self.run_data(data)[0].returncode,2,(n,wire[-8:]))
        # Modern LZW still rejects continued data after a full table.
        wire=lzw_codes([256]+[i%256 for i in range(3840)]+[257])
        data,_=make_tiff(3840,1,rows=1,compression=5,encoded_block=lambda _:wire)
        self.assertEqual(self.run_data(data)[0].returncode,2)
    def test_original_quad(self):
        src=ROOT/'tests/upstream/quad-lzw-compat.tiff';data=src.read_bytes()
        self.assertEqual(len(data),214342)
        self.assertEqual(data[:4],b'MM\0*')
        ifd=int.from_bytes(data[4:8],'big');tags={}
        for i in range(int.from_bytes(data[ifd:ifd+2],'big')):
            at=ifd+2+12*i;tag,kind,n=struct.unpack_from('>HHI',data,at)
            size={3:2,4:4}.get(kind)
            if size:
                pos=at+8 if n*size<=4 else int.from_bytes(data[at+8:at+12],'big')
                tags[tag]=[int.from_bytes(data[pos+k*size:pos+(k+1)*size],'big') for k in range(n)]
        decoded=[];traces=[]
        for offset,count in zip(tags[273],tags[279]):
            raw,trace=reference_stream(data[offset:offset+count]);decoded.append(raw);traces.append(trace)
            self.assertTrue(trace['complete_eoi']);self.assertLess(trace['remaining_bits'],8)
        expected=b''.join(decoded);self.assertEqual(len(traces),77)
        self.assertEqual(sum(t['frozen_steps'] for t in traces),4549)
        self.assertEqual(hashlib.sha256(expected).hexdigest(),'cd515a3f3a51ac88da3df62a657d892e97942992b091cd7f16900721ae41cb9b')
        meta=self.check(data,expected);self.oracle(expected)
        for flag,value in [('--work',meta['work_used']-1),('--max-output',len(expected)-1)]:
            p,_=self.run_data(data,flag,value);self.assertEqual(p.returncode,2);self.assertIn(b'Limit',p.stderr)
        self.assertEqual(self.run_data(data,'--work',meta['work_used'])[0].returncode,0)
        report=dict(file=src.name,input_sha256=hashlib.sha256(data).hexdigest(),strips=77,
            frozen_steps=4549,output_bytes=len(expected),pixel_sha256=hashlib.sha256(expected).hexdigest(),
            metadata=meta,comparison='independent Python string dictionary and libtiff native RGB both exact',trace=traces)
        (ROOT/'target/quad-lzw-compat-comparison.json').write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':unittest.main(verbosity=2)
