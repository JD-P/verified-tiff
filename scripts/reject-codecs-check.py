#!/usr/bin/env python3
"""LERC / full static WebP contracts must reject semantic and acceptance bugs."""
import os,pathlib,shutil,subprocess
root=pathlib.Path(__file__).resolve().parents[1]
changes={
 'lerc-rounding':('lerc_numeric.rs','q+if rem>half || (rem==half && q%2==1){1}else{0}','q+if rem>half || (rem==half && q%2==0){1}else{0}','lerc_numeric','round','postcondition not satisfied'),
 'webp-group':('webp_pixels.rs','Ok(meta[cell][0] as usize*256+meta[cell][1] as usize)','Ok(meta[cell][1] as usize*256+meta[cell][0] as usize)','webp_pixels','select','postcondition not satisfied'),
 'lerc-checksum':('lerc_header.rs','if sum!=read32(s,10,true)?{return Err(Error::Invalid);}','if false{return Err(Error::Invalid);}','lerc_header','inspect','postcondition not satisfied'),
 'webp-predictor':('webp_predict.rs','clip(l as i32+t as i32-tl as i32)','clip(l as i32+t as i32+tl as i32)','webp_predict','predict_one','postcondition not satisfied'),
 'webp-palette':('webp_inverse.rs','(out[source][1] as usize/(bit_divisor(shift) as usize))%modulus','(out[source][0] as usize/(bit_divisor(shift) as usize))%modulus','webp_inverse','index','postcondition not satisfied'),
 'webp-cross-color':('webp_predict.rs','let d=color_delta(m[2],p[1]);','let d=color_delta(m[0],p[1]);','webp_predict','inverse_color','postcondition not satisfied'),
 'vp8-bool':('vp8_bool.rs','let take=t.value>=split;','let take=t.value>split;','vp8_bool','bit','postcondition not satisfied'),
 'vp8-color':('vp8_color.rs','clip(yy+u as i32*33050/256-17685),255]','clip(yy+u as i32*33050/256-17685),254]','vp8_color','rgb','postcondition not satisfied'),
 'webp-alpha':('webp_alpha.rs','else if i%w==0{p[i-w]}','else if i%w==0{0}','webp_alpha','predict','postcondition not satisfied'),
}
def reject(name,dest,module,function,reason):
    log=root/'target'/('reject-codecs-'+name+'.log')
    with log.open('w') as f:r=subprocess.run([os.environ['VERUS'],'--crate-type','lib','--triggers-mode','silent','--rlimit','100','--verify-only-module',module,'--verify-function',function,str(dest/'lib.rs')],cwd=root,stdout=f,stderr=subprocess.STDOUT)
    assert r.returncode!=0 and reason in log.read_text(),(name,log.read_text())
    print('LERC/WebP semantic gate: rejected broken '+name,flush=True)
for name,(file,before,after,module,function,reason) in changes.items():
    dest=root/'target'/('reject-codecs-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
    p=dest/file;s=p.read_text();assert s.count(before)==1,name;p.write_text(s.replace(before,after));reject(name,dest,module,function,reason)
for name,module,function in [('lerc-acceptance','native_lerc','decode'),('webp-acceptance','native_webp','decode'),('vp8-acceptance','native_vp8','decode_yuv')]:
    dest=root/'target'/('reject-codecs-'+name);shutil.copytree(root/'src',dest,dirs_exist_ok=True)
    p=dest/(module+'.rs');s=p.read_text();a=s.index('pub fn '+function+'(');a=s.index('\n{',a);depth=0
    for b in range(a+1,len(s)):
        if s[b]=='{':depth+=1
        elif s[b]=='}':
            depth-=1
            if depth==0:break
    p.write_text(s[:a]+'\n{Err(Error::Invalid)}'+s[b+1:]);reject(name,dest,module,function,'postcondition not satisfied')
