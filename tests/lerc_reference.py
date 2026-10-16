"""Pinned Esri LERC 4.0.0 test-only encoder and decoder."""
import ctypes as C
import pathlib

ROOT=pathlib.Path(__file__).resolve().parents[1]
LIB=C.CDLL(str(ROOT/'target/lerc-reference/libLerc.so'))
P=C.c_void_p; I=C.c_int; U=C.c_uint; D=C.c_double
LIB.lerc_computeCompressedSizeForVersion.argtypes=[P,I,U,I,I,I,I,I,P,D,C.POINTER(U)]
LIB.lerc_encodeForVersion.argtypes=[P,I,U,I,I,I,I,I,P,D,P,U,C.POINTER(U)]
LIB.lerc_decode.argtypes=[P,U,I,P,I,I,I,I,U,P]

def encode(raw,kind,w,h,depth=1,mask=None,error=0,version=4,bands=1):
    source=C.create_string_buffer(raw)
    valid=C.create_string_buffer(bytes(mask)) if mask is not None else None
    args=(source,version,kind,depth,w,h,bands,int(mask is not None),valid,error)
    size=U(); status=LIB.lerc_computeCompressedSizeForVersion(*args,C.byref(size))
    assert status==0,('compute',status)
    out=C.create_string_buffer(size.value);written=U()
    status=LIB.lerc_encodeForVersion(*args,out,size,C.byref(written))
    assert status==0,('encode',status)
    return out.raw[:written.value]

def decode(blob,kind,w,h,depth=1,bands=1):
    source=C.create_string_buffer(blob)
    width=(1,1,2,2,4,4,4,8)[kind]
    out=C.create_string_buffer(w*h*depth*width*bands);mask=C.create_string_buffer(w*h)
    status=LIB.lerc_decode(source,len(blob),1,mask,depth,w,h,bands,kind,out)
    assert status==0,('decode',status)
    return out.raw,mask.raw
