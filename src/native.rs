//! Container-independent native strip/tile decoding. No color normalization.
use super::*;
verus! {

pub open spec fn work_bound(count:int,cap:int) -> int {8*count+3*cap+64}
pub fn work_limit(count:usize,cap:usize) -> (r:Result<usize,Error>)
    ensures r.is_ok()==(work_bound(count as int,cap as int)<=usize::MAX),
        r.is_ok() ==> r.unwrap()==work_bound(count as int,cap as int),
{add(add(mul(count,8)?,mul(cap,3)?)?,64)}

pub closed spec fn accepted(s:Seq<u8>,codec:u16,stride:usize,needed:usize,cap:usize) -> bool {
    &&& stride>0 && 0<needed<=cap
    &&& work_bound(s.len() as int,cap as int)<=usize::MAX
    &&& if codec==1 {needed<=s.len()}
        else if codec==32773 {pb_valid(s,0,s.len() as int,stride as int,needed as int)}
        else if codec==5 {lzw_valid(s,0,s.len() as int,needed as int)}
        else if codec==8 || codec==32946 {
            let v=deflate::result(s,0,s.len() as int,cap as int,(8*s.len()+2*cap+64) as nat);
            v.is_some() && needed<=v.unwrap().len()
        } else {false}
}
pub closed spec fn decoded_byte(s:Seq<u8>,codec:u16,cap:usize,k:int) -> u8 {
    if codec==1 {s[k]} else if codec==32773 {pb_byte(s,0,s.len() as int,k)}
    else if codec==5 {lzw_byte(s,0,s.len() as int,k)}
    else {deflate::result(s,0,s.len() as int,cap as int,(8*s.len()+2*cap+64) as nat).unwrap()[k]}
}

/// Writes exactly `needed` native bytes; Deflate may validate bounded final-strip
/// padding up to `cap`. Failure may change this prefix, never its suffix.
/// Callers requiring atomic output use a temporary segment buffer.
pub fn decode(s:&[u8],codec:u16,stride:usize,needed:usize,cap:usize,
    output:&mut [u8],budget:&mut Budget) -> (r:Result<(),Error>)
    ensures final(output).len()==old(output).len(),
        final(budget).remaining<=old(budget).remaining,
        r.is_ok()==(accepted(s@,codec,stride,needed,cap) && old(output).len()>=needed
            && old(budget).remaining as int>=work_bound(s.len() as int,cap as int)),
        r.is_ok() ==> final(budget).remaining as int+work_bound(s.len() as int,cap as int)==old(budget).remaining,
        r.is_ok() ==> forall|k:int| 0<=k<needed ==> final(output)@[k]==decoded_byte(s@,codec,cap,k),
        forall|k:int| needed<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
{
    reveal(accepted);reveal(decoded_byte);
    if stride==0 || needed==0 || needed>cap {return Err(Error::Invalid);}
    if output.len()<needed {return Err(Error::OutputTooSmall);}
    let quota=work_limit(s.len(),cap)?;
    spend(budget,quota)?;
    if codec==1 {
        if s.len()<needed {return Err(Error::Truncated);}
        copy_byte_run(s,0,output,0,needed,false);
        assert forall|k:int| 0<=k<needed implies output@[k]==decoded_byte(s@,codec,cap,k) by {
            assert(output@[0int+k]==s@[0int+k]);
        }
    } else if codec==32773 {
        let mut probe=Budget {remaining:s.len()};
        pb_validate(s,0,s.len(),stride,needed,&mut probe)?;
        let mut cursor=PbCursor::new(s,0,s.len(),stride,needed);
        let mut i=0usize;
        while i<needed
            invariant i<=needed,needed<=output.len(),output.len()==old(output).len(),
                cursor.valid(s@),cursor._start==0,cursor.end==s.len(),cursor._stride==stride,
                cursor._needed==needed,cursor.emitted==i,
                forall|k:int| 0<=k<i ==> output@[k]==pb_byte(s@,0,s.len() as int,k),
                forall|k:int| needed<=k<output.len() ==> output@[k]==old(output)@[k],
            decreases needed-i,
        {
            let n=cursor.take(s,output,i,needed-i);
            assert forall|k:int| i<=k<i+n implies output@[k]==pb_byte(s@,0,s.len() as int,k) by {
                assert(output@[i as int+(k-i)]==pb_byte(s@,0,s.len() as int,i as int+(k-i)));
            }
            i=i+n;
        }
        assert forall|k:int| 0<=k<needed implies output@[k]==decoded_byte(s@,codec,cap,k) by {}
    } else if codec==5 {
        let mut probe=Budget {remaining:s.len()};
        lz_validate(s,0,s.len(),needed,&mut probe)?;
        let mut cursor=LzCursor::new(s,0,s.len(),needed);
        let mut i=0usize;
        while i<needed
            invariant i<=needed,needed<=output.len(),output.len()==old(output).len(),
                cursor.valid(s@),cursor._start==0,cursor.bits.end==s.len(),
                cursor._needed==needed,cursor.emitted==i,
                forall|k:int| 0<=k<i ==> output@[k]==lzw_byte(s@,0,s.len() as int,k),
                forall|k:int| needed<=k<output.len() ==> output@[k]==old(output)@[k],
            decreases needed-i,
        {let byte=cursor.next(s);output[i]=byte;i=i+1;}
        assert forall|k:int| 0<=k<needed implies output@[k]==decoded_byte(s@,codec,cap,k) by {}
    } else if codec==8 || codec==32946 {
        let fuel=quota-cap;
        let n=deflate::process(s,0,s.len(),cap,fuel,output,needed)?;
        if n<needed {return Err(Error::Truncated);}
        assert forall|k:int| 0<=k<needed implies output@[k]==decoded_byte(s@,codec,cap,k) by {}
    } else {return Err(Error::Unsupported);}
    Ok(())
}
/// Complete bounded zlib expansion when a container embeds another codec and
/// the expanded length is not available until that codec's header is read.
pub closed spec fn inflated(s:Seq<u8>,cap:usize) -> Option<Seq<u8>> {
    deflate::result(s,0,s.len() as int,cap as int,(8*s.len()+2*cap+64) as nat)
}
pub fn inflate(s:&[u8],cap:usize,output:&mut[u8],budget:&mut Budget) -> (r:Result<usize,Error>)
    ensures final(output).len()==old(output).len(),final(budget).remaining<=old(budget).remaining,
        r.is_ok()==(old(output).len()>=cap && inflated(s@,cap).is_some() && work_bound(s.len() as int,cap as int)<=old(budget).remaining),
        r.is_ok() ==> r.unwrap()<=cap && final(output)@.take(r.unwrap() as int)==inflated(s@,cap).unwrap(),
        r.is_ok() ==> final(budget).remaining as int+work_bound(s.len() as int,cap as int)==old(budget).remaining,
        forall|k:int|cap<=k<old(output).len() ==> final(output)@[k]==old(output)@[k],
{
    reveal(inflated);
    if output.len()<cap{return Err(Error::OutputTooSmall);}
    let quota=work_limit(s.len(),cap)?;spend(budget,quota)?;
    let n=deflate::process(s,0,s.len(),cap,quota-cap,output,cap)?;
    assert(output@.take(n as int) =~= inflated(s@,cap).unwrap());Ok(n)
}
/// Legacy JPEG tables and scan selectors from a bounded shared header.
/// No entropy stream is decoded by this helper.
pub closed spec fn jpeg_shared_model(s:Seq<u8>) -> Option<Seq<usize>> {
    let h=crate::jpeg_container::header_model(s,0,s.len() as int,Seq::new(32,|i:int|0usize));
    if h.is_some() && h.unwrap().v[31]==1 {Some(h.unwrap().v)} else {None}
}
pub fn jpeg_shared(s:&[u8],budget:&mut Budget) -> (r:Result<[usize;32],Error>)
    ensures r.is_ok()==(jpeg_shared_model(s@).is_some() && old(budget).remaining as int>=2*s.len()+1),
        r.is_ok() ==> r.unwrap()@==jpeg_shared_model(s@).unwrap(),
        r.is_ok() ==> final(budget).remaining as int+2*s.len()+1==old(budget).remaining,
        final(budget).remaining<=old(budget).remaining,
{
    reveal(jpeg_shared_model);
    let quota=add(mul(s.len(),2)?,1)?;spend(budget,quota)?;
    let z=crate::jpeg::empty();
    assert(z@ =~= Seq::new(32,|i:int|0usize));
    assert(crate::jpeg_header::shape(z@) && crate::jpeg_header::tables_ok(s@,z@));
    let h=crate::jpeg_container::read_header(s,0,s.len(),z)?;
    if h.v[31]!=1 {return Err(Error::Unsupported);}
    Ok(h.v)
}

pub closed spec fn ojpeg_scan_valid(s:Seq<u8>,ch:usize,ids:[u8;3],selectors:[u8;3]) -> bool {
    &&& (ch==1 || ch==3) && s.len()>=8+2*ch
    &&& s[0]==255 && s[1]==218 && word16(s,2,false)==6+2*ch && s[4]==ch
    &&& s[5]==ids[0] && s[6]==selectors[0]
    &&& (ch==3 ==> s[7]==ids[1] && s[8]==selectors[1] && s[9]==ids[2] && s[10]==selectors[2])
    &&& s[5+2*ch]==0 && s[6+2*ch]==63 && s[7+2*ch]==0
}
pub closed spec fn ojpeg_renumbered(s:Seq<u8>,ch:usize) -> Seq<u8> {
    let a=s.update(5,0).update(6,0);
    if ch==3 {a.update(7,1).update(8,17).update(9,2).update(10,34)} else {a}
}
/// Relabel a baseline SOS to the per-component copies of its shared tables.
/// Every source selector is checked; entropy bytes and all other bytes remain.
pub fn renumber_ojpeg_scan(s:&mut [u8],ch:usize,ids:[u8;3],selectors:[u8;3]) -> (r:Result<(),Error>)
    ensures r.is_ok()==ojpeg_scan_valid(old(s)@,ch,ids,selectors),
        r.is_ok() ==> final(s)@==ojpeg_renumbered(old(s)@,ch),
        r.is_err() ==> final(s)@==old(s)@,
{
    reveal(ojpeg_scan_valid);reveal(ojpeg_renumbered);
    if ch!=1 && ch!=3 {return Err(Error::Unsupported);}
    if s.len()<8+2*ch {return Err(Error::Truncated);}
    if s[0]!=255 || s[1]!=218 || read16(s,2,false)? as usize!=6+2*ch || s[4] as usize!=ch ||
        s[5]!=ids[0] || s[6]!=selectors[0] ||
        (ch==3 && (s[7]!=ids[1] || s[8]!=selectors[1] || s[9]!=ids[2] || s[10]!=selectors[2])) ||
        s[5+2*ch]!=0 || s[6+2*ch]!=63 || s[7+2*ch]!=0 {return Err(Error::Invalid);}
    s[5]=0;s[6]=0;
    if ch==3 {s[7]=1;s[8]=17;s[9]=2;s[10]=34;}
    Ok(())
}

}
