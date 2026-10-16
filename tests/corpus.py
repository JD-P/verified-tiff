"""Measure first-main-image compatibility on every original libtiff fixture.
This is not a count of libtiff regression-test assertions or page coverage.
"""
import hashlib,json,pathlib,struct,subprocess,tempfile
from fixtures import canonical_alpha
from extended import pixels
from jpeg_fixtures import oracle as jpeg_oracle
ROOT=pathlib.Path(__file__).resolve().parents[1]
BIN=ROOT/'target/debug/verified-tiff'

def tag(data,key,default):
    order='<' if data[:2]==b'II' else '>'
    off=struct.unpack_from(order+'I',data,4)[0];n=struct.unpack_from(order+'H',data,off)[0]
    for j in range(n):
        p=off+2+12*j;t,kind,count=struct.unpack_from(order+'HHI',data,p)
        if t==key:return struct.unpack_from(order+('H' if kind==3 else 'I'),data,p+8)[0]
    return default

def run():
    cases=[]
    with tempfile.TemporaryDirectory() as tmp:
        tmp=pathlib.Path(tmp)
        for src in sorted((ROOT/'.tools/tiff-4.7.1/test/images').iterdir()):
            if src.suffix.lower() not in ('.tif','.tiff'):continue
            out=tmp/'ours.pnm';ref=tmp/'oracle.raw'
            result=subprocess.run([str(BIN),str(src),str(out)],capture_output=True,timeout=30)
            case={'file':src.name,'input_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'accepted':result.returncode==0}
            if result.returncode:case['error']=result.stderr.decode().strip()
            else:
                meta=json.loads(result.stdout);actual=pixels(out);data=src.read_bytes();photo=tag(data,262,1)
                if meta['compression'] in (6,7):
                    expected=jpeg_oracle(src,ref)
                elif meta['bits_per_sample']==1 and photo!=3:
                    oracle=ROOT/'target/libtiff-oracle'
                    q=subprocess.run([str(oracle),str(src),str(ref),'0'],capture_output=True,timeout=30)
                    assert q.returncode==0,(src,q.stderr)
                    expected=pixels(ref)[::3]
                else:
                    q=subprocess.run([str(ROOT/'target/native-oracle'),str(src),str(ref),'0'],capture_output=True,timeout=30)
                    assert q.returncode==0,(src,q.stderr)
                    expected=ref.read_bytes()
                    if meta['source_alpha']!='none':expected=canonical_alpha(expected,meta['channels'],photo==0,1 if meta['source_alpha']=='associated' else 2)
                    elif photo==0:expected=bytes(255-v for v in expected)
                assert len(actual)==len(expected)
                diffs=[abs(a-b) for a,b in zip(actual,expected)]
                if meta['compression'] in (6,7):
                    assert max(diffs)<=3 and sum(diffs)/len(diffs)<0.1,(src.name,'JPEG tolerance')
                    case.update(comparison='JPEG nearest chroma, bounded integer-IDCT difference',max_channel_error=max(diffs),mean_absolute_error=sum(diffs)/len(diffs))
                else:assert actual==expected,(src.name,'pixel mismatch')
                case.update(metadata=meta,pixels_match=actual==expected,pixel_sha256=hashlib.sha256(actual).hexdigest())
            cases.append(case)
    accepted=sum(c['accepted'] for c in cases)
    report={'libtiff':'4.7.1','measurement':'original files, first main image only, native pixels compared','accepted':accepted,'total':len(cases),'percentage':100*accepted/len(cases),'cases':cases}
    (ROOT/'target/original-corpus.json').write_text(json.dumps(report,indent=2)+'\n')
    print(f'Original corpus: {accepted}/{len(cases)} = {report["percentage"]:.1f}%; lossless pixels exact; JPEG maximum channel error <=3')

if __name__=='__main__':run()
