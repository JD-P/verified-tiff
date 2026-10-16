"""Adversarial public ABI cases for the newly checked adapter decisions."""
import struct,itertools,subprocess,tempfile,pathlib,unittest
from coverage import assemble,OUT
class AdapterProofs(unittest.TestCase):
 def test_schema_mismatch_and_boundary_safety(self):
  for le,big,(kind,value) in itertools.product([False,True],[False,True],[(12,[1.0]),(4,[65536]),(3,[1,2])]):
   if kind==12:value=struct.pack(("<" if le else ">")+"d",1.0)
   tags=[(256,4,[2]),(257,4,[1]),(258,3,[8]),(259,3,[1]),(262,3,[1]),(273,16 if big else 4,('SEG',[b'\x11\x29'])),(277,3,[1]),(278,4,[1]),(279,16 if big else 4,[2]),(65001,kind,value)]
   with tempfile.TemporaryDirectory() as d:
    f=pathlib.Path(d)/'input.tif';f.write_bytes(assemble([tags],big=big,le=le)[0]);subprocess.run([str(OUT/'adapter-proofs'),str(f)],check=True,capture_output=True)
if __name__=='__main__':unittest.main()
