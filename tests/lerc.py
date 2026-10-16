"""Full Lerc2 v4 sample/geometry/mask/entropy differential tests."""
import pathlib,random,struct,subprocess,unittest
from lerc_reference import encode,decode,ROOT

class Lerc(unittest.TestCase):
    def test_reference_matrix(self):
        target=ROOT/'target/lerc-cases';target.mkdir(exist_ok=True)
        rng=random.Random(104729);cases=0
        for kind,fmt in enumerate('bBhHiIfd'):
            for depth in (1,3,7):
                for w,h in ((1,1),(17,19),(65,33)):
                    for pattern in ('zero','constant','bands','ramp','noise','sparse'):
                        for masked in (False,True):
                            values=[]
                            bits=8*struct.calcsize(fmt)
                            for i in range(w*h):
                                for d in range(depth):
                                    v={'zero':0,'constant':5,'bands':d*3+1,'ramp':i//w+i%w+d*2,'noise':rng.randrange(256),'sparse':rng.choice((0,0,0,1,19,101))}[pattern]
                                    if kind<6:
                                        v=v%2**bits
                                        if kind%2==0:v=(v+2**(bits-1))%2**bits-2**(bits-1)
                                    else:v=v/7 if pattern in ('ramp','noise') else float(v)
                                    values.append(v)
                            raw=struct.pack('<'+fmt*len(values),*values)
                            mask=[int(i%11!=0 and i%13!=0) for i in range(w*h)] if masked else None
                            error=0.015 if kind>=6 and pattern=='ramp' else 0
                            blob=encode(raw,kind,w,h,depth,mask,error)
                            expected,valid=decode(blob,kind,w,h,depth)
                            name=f'{kind}-{depth}-{w}-{h}-{pattern}-{int(masked)}'
                            path=target/(name+'.lerc');path.write_bytes(blob)
                            result=subprocess.run([str(ROOT/'target/debug/examples/lerc-native'),str(path)],capture_output=True)
                            self.assertEqual(result.returncode,0,(name,result.stderr.decode()))
                            self.assertEqual(result.stdout[:len(expected)],expected,name)
                            bitmap=result.stdout[len(expected):]
                            self.assertEqual(len(bitmap),(w*h+7)//8,name)
                            self.assertEqual(bytes((bitmap[i//8]>>(7-i%8))&1 for i in range(w*h)),valid,name)
                            cases+=1
        print(f'LERC: {cases} upstream encoded/decoded cases matched exact native samples and masks')

    def test_extreme_samples_and_depth(self):
        import tempfile
        cases=0
        with tempfile.TemporaryDirectory() as td:
            path=pathlib.Path(td)/'edge.lerc'
            for kind,fmt in enumerate('bBhHiIfd'):
                bits=8*struct.calcsize(fmt)
                if kind<6:
                    lo=-(1<<(bits-1)) if kind%2==0 else 0
                    hi=(1<<(bits-1))-1 if kind%2==0 else (1<<bits)-1
                    values=[lo,lo+1,-1 if lo<0 else 1,0,1,hi-1,hi]
                elif kind==6:values=[-3.4e38,-1e10,-0.0,0.0,2**-149,2**-126,1e10,3.4e38]
                else:values=[-1.7e308,-1e100,-0.0,0.0,2**-1074,2**-1022,1e100,1.7e308]
                for depth in (1,64):
                    w,h=17,9;raw=struct.pack('<'+fmt*(w*h*depth),*(values[i%len(values)] for i in range(w*h*depth)))
                    for mask in (None,[0]*(w*h)):
                        blob=encode(raw,kind,w,h,depth,mask);expected,valid=decode(blob,kind,w,h,depth);path.write_bytes(blob)
                        run=subprocess.run([str(ROOT/'target/debug/examples/lerc-native'),str(path)],capture_output=True)
                        self.assertEqual(run.returncode,0,(kind,depth,mask is None,run.stderr));self.assertEqual(run.stdout[:len(expected)],expected,(kind,depth));cases+=1
        print(f'LERC edge/depth cases: {cases} exact sample comparisons')

if __name__=='__main__':unittest.main()
