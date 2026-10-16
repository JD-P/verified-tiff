"""LERC strips/tiles, byte orders, sample planes, wrappers and mask policies."""
import itertools,pathlib,struct,subprocess,sys,tempfile,unittest,zlib
from coverage import assemble,OUT,ROOT
sys.path.insert(0,str(ROOT/'tests'))
from lerc_reference import encode,decode
from zstd_reference import compress

def scene(kind=1,ch=3,big=False,le=True,tiled=False,separate=False,wrapper=0,alpha=False,banded=False):
    w,h=19,17;bw,bh=(16,16) if tiled else (w,5)
    bits=(8,8,16,16,32,32,32,64)[kind];width=bits//8;fmt='bBhHiIfd'[kind]
    segments=[];expected=[];columns=(w+bw-1)//bw;lines=(h+bh-1)//bh
    channels=1 if separate else ch
    for plane in range(ch if separate else 1):
        for by in range(lines):
            for bx in range(columns):
                rows=bh if tiled else min(bh,h-by*bh);n=bw*rows
                def val(p,c):
                    x=bx*bw+p%bw;y=by*bh+p//bw
                    v=x*13+y*7+c*23
                    if kind>=6:return (v-79)/7
                    v%=2**bits
                    return (v+2**(bits-1))%2**bits-2**(bits-1) if kind%2==0 else v
                def mask(c):return [int((p+c*5)%13!=0) for p in range(n)]
                depth=channels-1 if alpha else channels
                blob=bytearray();native=bytearray(n*channels*width)
                for b in range(channels if banded else 1):
                    nd=1 if banded else depth;valid=mask(b) if kind>=6 or alpha else None
                    values=[val(p,plane if separate else b if banded else c) for p in range(n) for c in range(nd)]
                    raw=struct.pack('<'+fmt*len(values),*values)
                    part=encode(raw,kind,bw,rows,nd,valid,0.01 if kind>=6 else 0)
                    oracle,valid=decode(part,kind,bw,rows,nd);blob.extend(part)
                    for p in range(n):
                        for c in range(nd):
                            v=oracle[(p*nd+c)*width:(p*nd+c+1)*width]
                            if kind>=6 and not valid[p]:v=struct.pack('<I',0x7fc00000) if kind==6 else struct.pack('<Q',0x7ff8000000000000)
                            target=(p*channels+(b if banded else c))*width
                            native[target:target+width]=v
                        if alpha:native[p*channels+channels-1]=255 if valid[p] else 0
                blob=bytes(blob);encoded=zlib.compress(blob) if wrapper==1 else compress(blob) if wrapper==2 else blob
                segments.append(encoded);expected.append(native)
    ot=16 if big else 4
    tags=[(256,4,[w]),(257,4,[h]),(258,3,[bits]*ch),(259,3,[34887]),(262,3,[2 if ch>=3 else 1]),
          (277,3,[ch]),(284,3,[2 if separate else 1]),(339,3,[3 if kind>=6 else 2 if kind%2==0 else 1]*ch),
          (50674,4,[4,wrapper]),((324 if tiled else 273),ot,('SEG',segments)),((325 if tiled else 279),ot,list(map(len,segments)))]
    tags.extend([(322,4,[bw]),(323,4,[bh])] if tiled else [(278,4,[bh])])
    if alpha:tags.append((338,3,[2]))
    return assemble([tags],big,le)[0],b''.join(expected)

class LercCompatibility(unittest.TestCase):
    def test_native_matrix(self):
        with tempfile.TemporaryDirectory() as temp:
            path=pathlib.Path(temp);source=path/'in.tif';out=path/'out.bin';cases=0
            for kind,big,le,tiled,separate,wrapper in itertools.product(range(8),(False,True),(False,True),(False,True),(False,True),range(3)):
                blob,expected=scene(kind,big=big,le=le,tiled=tiled,separate=separate,wrapper=wrapper)
                source.write_bytes(blob)
                run=subprocess.run([str(OUT/'dump'),str(source),'native',str(out),'0','1'],capture_output=True)
                self.assertEqual(run.returncode,0,(kind,big,le,tiled,separate,wrapper,run.stderr))
                self.assertEqual(out.read_bytes(),expected,(kind,big,le,tiled,separate,wrapper))
                ref=subprocess.run([str(OUT/'dump-libtiff-lerc'),str(source),'native',str(out),'0','1'],capture_output=True)
                self.assertEqual(ref.returncode,0,ref.stderr)
                upstream=out.read_bytes();width=(1,1,2,2,4,4,4,8)[kind]
                # libtiff applies its generic TIFF byte swap after LERC has
                # already returned native typed words. The payload has its
                # own fixed endian encoding. Undo that extra reference swap.
                if not le:upstream=b''.join(upstream[i:i+width][::-1] for i in range(0,len(upstream),width))
                self.assertEqual(upstream,expected,(kind,big,le,tiled,separate,wrapper));cases+=1
            print(f'LERC C API: {cases} native TIFF combinations matched independent samples')
    def test_alpha_and_float_bands(self):
        with tempfile.TemporaryDirectory() as temp:
            path=pathlib.Path(temp);source=path/'in.tif';out=path/'out.bin'
            for wrapper,tiled in itertools.product(range(3),(False,True)):
                for kind,ch,alpha,banded in ((1,4,True,False),(6,3,False,True),(7,3,False,True)):
                    blob,expected=scene(kind,ch=ch,wrapper=wrapper,tiled=tiled,alpha=alpha,banded=banded)
                    source.write_bytes(blob)
                    run=subprocess.run([str(OUT/'dump'),str(source),'native',str(out),'0','1'],capture_output=True)
                    self.assertEqual(run.returncode,0,run.stderr);self.assertEqual(out.read_bytes(),expected)
                    ref=subprocess.run([str(OUT/'dump-libtiff-lerc'),str(source),'native',str(out),'0','1'],capture_output=True)
                    self.assertEqual(ref.returncode,0,(kind,wrapper,ref.stderr));self.assertEqual(out.read_bytes(),expected)

    def test_shared_mask_bands(self):
        w,h,ch=17,9,3;n=w*h
        with tempfile.TemporaryDirectory() as temp:
            source=pathlib.Path(temp)/'in.tif';out=pathlib.Path(temp)/'out.bin'
            for kind,wrapper in itertools.product((6,7),range(3)):
                fmt='f' if kind==6 else 'd';width=4 if kind==6 else 8
                valid=[int(p%13!=0) for p in range(n)]
                raw=struct.pack('<'+fmt*(n*ch),*((p*11+b*37)/7 for b in range(ch) for p in range(n)))
                blob=encode(raw,kind,w,h,mask=valid,error=0.01,bands=ch)
                # Esri omits the repeated mask in subsequent band blobs.
                offset=struct.unpack_from('<I',blob,34)[0]
                self.assertEqual(struct.unpack_from('<I',blob,offset+66)[0],0)
                ref,mask=decode(blob,kind,w,h,bands=ch);expected=bytearray()
                for p in range(n):
                    for b in range(ch):
                        expected+=ref[(b*n+p)*width:(b*n+p+1)*width] if valid[p] else struct.pack('<I' if kind==6 else '<Q',0x7fc00000 if kind==6 else 0x7ff8000000000000)
                encoded=zlib.compress(blob) if wrapper==1 else compress(blob) if wrapper==2 else blob
                tags=[(256,4,[w]),(257,4,[h]),(258,3,[width*8]*ch),(259,3,[34887]),(262,3,[2]),(277,3,[ch]),(284,3,[1]),(339,3,[3]*ch),(50674,4,[4,wrapper]),(273,4,('SEG',[encoded])),(279,4,[len(encoded)]),(278,4,[h])]
                source.write_bytes(assemble([tags],False,True)[0])
                # Upstream libtiff rejects one shared mask across multiple float bands;
                # compare this additional accepted layout directly with Esri.
                for exe in ('dump',):
                    run=subprocess.run([str(OUT/exe),str(source),'native',str(out),'0','1'],capture_output=True)
                    self.assertEqual(run.returncode,0,(exe,run.stderr));self.assertEqual(out.read_bytes(),expected,(exe,kind,wrapper))

if __name__=='__main__':unittest.main(verbosity=2)
