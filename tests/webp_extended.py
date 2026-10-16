"""Independent VP8L bitstreams exercise every transform and entropy switching."""
import itertools,pathlib,random,struct,subprocess,tempfile,unittest
from webp_fixtures import Bits,table,riff,oracle,ROOT
EXE=ROOT/'target/debug/examples/webp-native'

def tables(bits,pixels):
    return [table(bits,n,[p[c] for p in pixels]) for n,c in ((280,1),(256,0),(256,2),(256,3))]+[table(bits,40,[0])]
def literals(bits,pixels,codes):
    for p in pixels:
        for group,c in enumerate((1,0,2,3)):bits.code(*codes[group][p[c]])
def auxiliary(bits,pixels):
    bits.put(0,1);codes=tables(bits,pixels);literals(bits,pixels,codes)
def stream(w,h,transforms,groups=1):
    b=Bits();width=w
    for kind,data in transforms:
        b.put(1,1);b.put(kind,2)
        if kind<2:
            b.put(0,3);cells=((width+3)//4)*((h+3)//4)
            auxiliary(b,[data[i%len(data)] for i in range(cells)])
        elif kind==3:
            b.put(len(data)-1,8);prev=(0,0,0,0);delta=[]
            for p in data:delta.append(tuple((a-v)%256 for a,v in zip(p,prev)));prev=p
            auxiliary(b,delta)
            per=8 if len(data)<=2 else 4 if len(data)<=4 else 2 if len(data)<=16 else 1
            width=(width+per-1)//per
    b.put(0,1);b.put(0,1);b.put(int(groups>1),1)
    if groups>1:
        b.put(0,3);across=(width+3)//4;down=(h+3)//4
        auxiliary(b,[(0,i%groups,0,255) for i in range(across*down)])
    pixels=[((i*31+61)%256,(i*17+3)%256,(i*71+19)%256,(i*13+97)%256) for i in range(width*h)]
    def group(i):return (((i//width)//4)*((width+3)//4)+(i%width)//4)%groups if groups>1 else 0
    codes=[tables(b,[p for i,p in enumerate(pixels) if group(i)==g]) for g in range(groups)]
    for i,p in enumerate(pixels):literals(b,[p],codes[group(i)])
    return riff(b'\x2f'+struct.pack('<I',(w-1)|((h-1)<<14)|(1<<28))+b.data())

class ExtendedWebP(unittest.TestCase):
    def test_explicit_transform_and_group_cases(self):
        cases=[]
        for mode in range(14):cases.append(('predictor'+str(mode),[(0,[(0,mode,0,255)])],1))
        cases.append(('cross-color',[(1,[(255,128,127,255),(67,223,129,255)])],1))
        for size in (1,2,3,4,5,16,17,256):
            palette=[((i*31)%256,(i*67)%256,(i*101)%256,(i*13+97)%256) for i in range(size)]
            cases.append(('palette'+str(size),[(3,palette)],1))
        options={0:(0,[(0,13,0,255)]),1:(1,[(255,128,127,255)]),2:(2,[]),3:(3,[(31,64,127,255),(127,31,64,71)])}
        for kinds in itertools.permutations(range(4)):
            cases.append(('order'+str(kinds),[options[k] for k in kinds],3))
        for groups in (2,3,17):cases.append(('entropy'+str(groups),[],groups))
        with tempfile.TemporaryDirectory() as tmp:
            tmp=pathlib.Path(tmp)
            for name,transforms,groups in cases:
                blob=stream(67,19,transforms,groups);expected=oracle(blob,tmp)
                run=subprocess.run([str(EXE),str(tmp/'segment.webp'),'67','19'],capture_output=True)
                self.assertEqual(run.returncode,0,(name,run.stderr));self.assertEqual(run.stdout,expected,name)
        print(f'General VP8L: {len(cases)} independent transform/group bitstreams matched libwebp')
if __name__=='__main__':unittest.main(verbosity=2)
