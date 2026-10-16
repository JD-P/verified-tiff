"""Exact-output LZW termination compatibility, with independent references."""
import hashlib, itertools, json, pathlib, subprocess, tempfile, unittest
from fixtures import make_tiff, lzw_codes, lzw_encode, change_inline
from extended import make_extended, pixels, ROOT, BIN, ORACLE


def reference_stream(encoded):
    """Test-only string dictionary; returns decoded bytes and terminal evidence.

    This analyzes all complete codes rather than stopping at TIFF's output size.
    A residual partial code is reported to the test, not accepted as an EOI.
    """
    bitstream=''.join(f'{b:08b}' for b in encoded)
    table=[bytes([i]) for i in range(256)]+[b'',b'']
    previous=None; pos=0; decoded=bytearray(); clears=0; codes=0
    while True:
        width=9 if len(table)<511 else 10 if len(table)<1023 else 11 if len(table)<2047 else 12
        if pos+width>len(bitstream):
            return bytes(decoded),dict(complete_eoi=False,consumed_bits=pos,
                remaining_bits=len(bitstream)-pos,residual_bits=bitstream[pos:],
                next_code_width=width,dictionary_entries=len(table),clears=clears,codes=codes)
        code=int(bitstream[pos:pos+width],2);pos+=width;codes+=1
        if code==256:
            table=[bytes([i]) for i in range(256)]+[b'',b''];previous=None;clears+=1
        elif code==257:
            return bytes(decoded),dict(complete_eoi=True,consumed_bits=pos,
                remaining_bits=len(bitstream)-pos,residual_bits=bitstream[pos:],
                next_code_width=width,dictionary_entries=len(table),clears=clears,codes=codes)
        else:
            assert code<=len(table) and (code<len(table) or previous is not None)
            phrase=table[code] if code<len(table) else previous+previous[:1]
            assert phrase
            decoded.extend(phrase)
            if previous is not None:
                assert len(table)<4096
                table.append(previous+phrase[:1])
            previous=phrase


class LzwTail(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.path=pathlib.Path(self.tmp.name)
    def tearDown(self):self.tmp.cleanup()
    def run_data(self,data,*flags):
        src=self.path/'in.tif';out=self.path/'out.pnm';src.write_bytes(data)
        result=subprocess.run([str(BIN),str(src),str(out),*map(str,flags)],capture_output=True,timeout=30)
        return result,out
    def check(self,data,expected):
        result,out=self.run_data(data)
        self.assertEqual(result.returncode,0,result.stderr)
        self.assertEqual(pixels(out),expected)
        return json.loads(result.stdout)
    def test_format_matrix(self):
        formats=[dict(bits=1,ch=1,white=v) for v in [False,True]]
        formats += [dict(bits=8,ch=c) for c in [1,3]]
        formats += [dict(bits=8,ch=c,alpha=a) for c,a in [(2,1),(4,2)]]
        formats += [dict(bits=16,ch=c) for c in [1,3]]
        formats += [dict(bits=b,ch=1,palette=True) for b in [1,4,8]]
        formats += [dict(bits=8,ch=3,separate=True),dict(bits=8,ch=4,separate=True,alpha=1)]
        formats += [dict(bits=64,ch=c,predictor=p) for c,p in [(1,1),(1,2),(3,2)]]
        cases=0
        for options,le,tiled in itertools.product(formats,[False,True],[False,True]):
            with self.subTest(options=options,le=le,tiled=tiled):
                data,expected,native=make_extended(w=17,h=19,le=le,tiled=tiled,
                    compression=5,transform=lambda encoded,j,raw:lzw_encode(raw,eoi=False),**options)
                meta=self.check(data,expected[0]);self.assertEqual(meta['compression'],5)
                # libtiff independently decodes every native sample, including
                # float64 predictor bits. Bilevel uses its RGBA renderer.
                ref=self.path/'reference.raw'
                bilevel=options['bits']==1 and not options.get('palette')
                oracle=ROOT/'target/libtiff-oracle' if bilevel else ORACLE
                self.assertTrue(oracle.exists(),'run scripts/upstream.py first')
                q=subprocess.run([str(oracle),str(self.path/'in.tif'),str(ref),'0'],capture_output=True,timeout=30)
                self.assertEqual(q.returncode,0,q.stderr)
                actual=pixels(ref)[::3] if bilevel else ref.read_bytes()
                self.assertEqual(actual,expected[0] if bilevel or options.get('palette') else native[0])
                cases+=1
        self.assertEqual(cases,64)
        print(f'LZW tail profile matrix: {cases} cases; independent pixels and live libtiff exact')
    def test_code_widths_padding_and_dictionary(self):
        widths=set();tails=set();cases=0
        for n,clears in itertools.product([*range(1,9),*range(254,262),*range(766,774),*range(1790,1798),3839],[1,2]):
            raw=bytes(i%256 for i in range(n));encoded=lzw_codes([256]*clears+list(raw))
            ref,trace=reference_stream(encoded);self.assertEqual(ref,raw)
            self.assertFalse(trace['complete_eoi']);self.assertLess(trace['remaining_bits'],8)
            widths.add(trace['next_code_width']);tails.add(trace['remaining_bits'])
            for fill in [0,(1<<trace['remaining_bits'])-1]:
                wire=encoded[:-1]+bytes([encoded[-1]|fill])
                data,_=make_tiff(n,1,rows=1,compression=5,encoded_block=lambda _,e=wire:e)
                self.check(data,raw);cases+=1
        self.assertEqual(widths,{9,10,11,12});self.assertEqual(tails,set(range(8)))
        for codes,raw in [([256,65,258,259,260],b'A'*10),
                          ([256,65,66,258,260],b'ABABABA'),
                          ([256,65,258,256,66,258],b'AAABBB')]:
            data,_=make_tiff(len(raw),1,rows=1,compression=5,encoded_block=lambda _,c=codes:lzw_codes(c))
            self.check(data,raw);cases+=1
        print(f'LZW tail width/padding/dictionary cases: {cases}')
    def test_reject_incomplete_pixels_and_extra_bytes(self):
        for n in [1,7,8,254,766,1790,3839]:
            raw=bytes(i%256 for i in range(n));encoded=lzw_codes([256,*raw])
            invalid=[encoded[:-1],encoded+b'\0',encoded+b'\xff',
                lzw_codes([*raw]),lzw_codes([256,*raw,256]),
                lzw_codes([256,*raw,0]),lzw_codes([256,*raw,257])+b'\0']
            for wire in invalid:
                data,_=make_tiff(n,1,rows=1,compression=5,encoded_block=lambda _,e=wire:e)
                p,_=self.run_data(data);self.assertEqual(p.returncode,2,(n,wire[-8:],p.stderr))
            # All but the final literal is a complete stream, but insufficient
            # for the declared geometry. A future dictionary reference is invalid.
            for wire in [lzw_codes([256,*raw[:-1]]),lzw_codes([256,42,259])]:
                data,_=make_tiff(n,1,rows=1,compression=5,encoded_block=lambda _,e=wire:e)
                self.assertEqual(self.run_data(data)[0].returncode,2)
        # A missing terminal marker after Clear is outside the compatibility
        # profile; complete EOI after any number of Clears remains supported.
        data,_=make_tiff(1,1,compression=5,encoded_block=lambda _:lzw_codes([256,42,256,256,257]))
        self.check(data,b'*')
    def test_original_single_strip(self):
        src=ROOT/'tests/upstream/lzw-single-strip.tiff';data=src.read_bytes()
        self.assertEqual(len(data),76264)
        self.assertEqual(hashlib.sha256(data).hexdigest(),'9cbff8fbe7a487da5d56e7e7566c0f3984e7092a3b1ccfe6bc1461449d008e4e')
        packed,trace=reference_stream(data[222:])
        self.assertEqual(len(packed),975*3122)
        self.assertEqual((trace['complete_eoi'],trace['remaining_bits'],trace['next_code_width'],trace['clears']),
                         (False,5,10,15))
        self.assertEqual(trace['residual_bits'],'01000')
        # Expand byte-padded WhiteIsZero scanlines independently of Rust/libtiff.
        table=[bytes(0 if value&(1<<(7-bit)) else 255 for bit in range(8)) for value in range(256)]
        expected=b''.join(b''.join(table[v] for v in packed[y*975:(y+1)*975])[:7795] for y in range(3122))
        meta=self.check(data,expected)
        self.assertEqual(meta['output_bytes'],7795*3122)
        ref=self.path/'reference.ppm'
        q=subprocess.run([str(ROOT/'target/libtiff-oracle'),str(src),str(ref),'0'],capture_output=True,timeout=30)
        self.assertEqual(q.returncode,0,q.stderr);self.assertEqual(pixels(ref)[::3],expected)
        # Complete the original truncated EOI (five missing bits plus padding).
        complete=change_inline(data+b'\x08',279,76043)
        self.assertTrue(reference_stream(complete[222:])[1]['complete_eoi'])
        self.check(complete,expected)
        # Deleting one more byte truncates required pixels. Adding zero turns
        # the partial EOI into Clear, which cannot use the missing-EOI exception.
        for invalid in [change_inline(data[:-1],279,76041),change_inline(data+b'\0',279,76043)]:
            self.assertEqual(self.run_data(invalid)[0].returncode,2)
        for flag,value in [('--work',meta['work_used']-1),('--max-output',len(expected)-1)]:
            p,_=self.run_data(data,flag,value);self.assertEqual(p.returncode,2);self.assertIn(b'Limit',p.stderr)
        self.assertEqual(self.run_data(data,'--work',meta['work_used'])[0].returncode,0)
        trace.update(file=src.name,input_sha256=hashlib.sha256(data).hexdigest(),
            encoded_bytes=76042,expanded_storage_bytes=len(packed),output_bytes=len(expected),
            pixel_sha256=hashlib.sha256(expected).hexdigest(),metadata=meta,
            comparison='independent Python string dictionary and libtiff Gray8 pixels both exact')
        (ROOT/'target/lzw-single-strip-comparison.json').write_text(json.dumps(trace,indent=2)+'\n')


if __name__=='__main__':unittest.main(verbosity=2)
