"""Pinned, independent Zstandard 1.5.7 test-only encoder/decoder."""
import ctypes,pathlib
ROOT=pathlib.Path(__file__).resolve().parents[1]
LIB=ROOT/'target/zstd-reference/lib/libzstd.so'
_z=None
def library():
 global _z
 if _z is not None:return _z
 z=ctypes.CDLL(str(LIB))
 for name,result,args in [('ZSTD_createCCtx',ctypes.c_void_p,[]),('ZSTD_freeCCtx',ctypes.c_size_t,[ctypes.c_void_p]),('ZSTD_CCtx_setParameter',ctypes.c_size_t,[ctypes.c_void_p,ctypes.c_int,ctypes.c_int]),('ZSTD_compressBound',ctypes.c_size_t,[ctypes.c_size_t]),('ZSTD_compress2',ctypes.c_size_t,[ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t,ctypes.c_void_p,ctypes.c_size_t]),('ZSTD_decompress',ctypes.c_size_t,[ctypes.c_void_p,ctypes.c_size_t,ctypes.c_void_p,ctypes.c_size_t]),('ZSTD_isError',ctypes.c_uint,[ctypes.c_size_t]),('ZSTD_getErrorName',ctypes.c_char_p,[ctypes.c_size_t]),('ZSTD_versionString',ctypes.c_char_p,[])]:
  f=getattr(z,name);f.restype=result;f.argtypes=args
 assert z.ZSTD_versionString()==b'1.5.7'
 _z=z;return z
def compress(data,level=3,content_size=True,checksum=True):
 z=library();c=z.ZSTD_createCCtx();assert c
 try:
  for param,value in [(100,level),(200,int(content_size)),(201,int(checksum))]:
   r=z.ZSTD_CCtx_setParameter(c,param,value);assert not z.ZSTD_isError(r),z.ZSTD_getErrorName(r)
  out=ctypes.create_string_buffer(z.ZSTD_compressBound(len(data)))
  n=z.ZSTD_compress2(c,out,len(out),data,len(data));assert not z.ZSTD_isError(n),z.ZSTD_getErrorName(n)
  return out.raw[:n]
 finally:z.ZSTD_freeCCtx(c)
def decompress(data,cap):
 z=library();out=ctypes.create_string_buffer(max(1,cap));n=z.ZSTD_decompress(out,cap,data,len(data))
 return None if z.ZSTD_isError(n) else out.raw[:n]
