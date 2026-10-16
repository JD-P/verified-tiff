#!/usr/bin/env python3
"""Semantic and acceptance gates for the second throughput pass."""
import pathlib,shutil,subprocess,os
root=pathlib.Path(__file__).resolve().parents[1]
changes={
    'bilevel':('lib.rs','    (base,n)\n}', '    (base,0)\n}', 'root','bilevel_span','postcondition not satisfied'),
    'window':('bit_window.rs','r as usize\n}', '0usize\n}', 'bit_window','read','postcondition not satisfied'),
    'transform':('jpeg_transform.rs','let dc=c[0]*11585;','let dc=c[0]*11584;','jpeg_transform','transform','assertion failed'),
    'identity':('extended_image.rs','dest.copy_from_slice(source);','/* deliberately omit copy */','extended::image','ExtendedImage::render_native','assertion failed'),
}
for name,(file,before,after,module,function,reason) in changes.items():
    dest=root/'target'/('reject-throughput-'+name)
    shutil.copytree(root/'src',dest,dirs_exist_ok=True)
    p=dest/file;s=p.read_text();assert s.count(before)==1,name;p.write_text(s.replace(before,after))
    log=root/'target'/('reject-throughput-'+name+'.log')
    with log.open('w') as f:
        r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100',
            *(['--verify-root'] if module=='root' else ['--verify-only-module',module]),'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
    assert r.returncode!=0 and reason in log.read_text(),(name,log.read_text())
    print('Throughput semantic gate: rejected broken '+name,flush=True)
# Replacing the public fused decoder with always-error must fail acceptance.
dest=root/'target/reject-throughput-combined';shutil.copytree(root/'src',dest,dirs_exist_ok=True)
p=dest/'extended_image.rs';s=p.read_text();start=s.index('pub fn decode_deflate_into(') if 'pub fn decode_deflate_into(' in s else s.index("pub fn decode_deflate_into<'a>")
body=s.index('\n{\n',start);depth=0
for i in range(body+1,len(s)):
    if s[i]=='{':depth+=1
    elif s[i]=='}':
        depth-=1
        if depth==0:break
p.write_text(s[:body]+'\n{\n    Err(Error::Invalid)\n}'+s[i+1:])
log=root/'target/reject-throughput-combined.log'
with log.open('w') as f:
    r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100',
        '--verify-only-module','extended::image','--verify-function','decode_deflate_into',str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
assert r.returncode!=0 and 'postcondition not satisfied' in log.read_text(),log.read_text()
print('Throughput acceptance gate: rejected always-error combined decoder',flush=True)
