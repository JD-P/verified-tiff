use super::*;
#[path="extended_image.rs"]
mod image;
pub use image::*;

verus! {

spec fn ext_bits(s:Seq<u8>,d:Directory,ch:int,le:bool) -> bool {
    let depth=bits_model(s,d,le);
    let photo=scalar_model(s,d,262,999,le);
    let alpha=scalar_model(s,d,338,0,le);
    ((photo==3 && ch==1 && (depth==1 || depth==4 || depth==8)) || (photo!=3 && (depth==8 || ((depth==16 || (depth==64 && (photo==1 || photo==2))) && alpha==0) || (depth==1 && ch==1 && alpha==0))))
    && match lookup(s,d,258,le) {None=>ch==1,Some(t)=>array_of(s,t,ch,depth,le)}
}

#[verifier::opaque]
spec fn ext_fields(s: Seq<u8>, d: Directory, le: bool, limits: Limits) -> bool {
    let w = scalar_model(s, d, 256, 0, le);
    let h = scalar_model(s, d, 257, 0, le);
    let ch = scalar_model(s, d, 277, 1, le);
    let photo = scalar_model(s, d, 262, 999, le);
    &&& scalar_valid(s, d, 256, le) && scalar_valid(s, d, 257, le)
    &&& scalar_valid(s, d, 277, le) && scalar_valid(s, d, 262, le)
    &&& 0 < w <= limits.max_dimension && 0 < h <= limits.max_dimension
    &&& (1 <= ch <= 4)
    &&& (if scalar_model(s,d,338,0,le)==0 {ch==1 || ch==3} else {ch==2 || ch==4})
    &&& ((ch <= 2 && (photo == 0 || photo == 1)) || (ch >= 3 && photo == 2) || (ch == 1 && photo == 3))
    &&& 0 <= w * h <= usize::MAX && 0 <= w * h * ch <= limits.max_output
    &&& scalar_valid(s, d, 259, le)
    &&& (scalar_model(s, d, 259, 1, le) == 1 || scalar_model(s, d, 259, 1, le) == 32773 || scalar_model(s, d, 259, 1, le) == 5 || scalar_model(s,d,259,1,le)==8 || scalar_model(s,d,259,1,le)==32946)
    &&& (scalar_model(s, d, 259, 1, le) != 1 ==> scalar_model(s, d, 266, 1, le) == 1)
    &&& scalar_valid(s, d, 274, le) && scalar_model(s, d, 274, 1, le) == 1
    &&& scalar_valid(s, d, 284, le) && (scalar_model(s, d, 284, 1, le) == 1 || scalar_model(s, d, 284, 1, le) == 2)
    &&& scalar_valid(s, d, 266, le)
    &&& scalar_model(s, d, 266, 1, le) == 1
    &&& scalar_valid(s, d, 317, le)
    &&& (scalar_model(s,d,317,1,le)==1 || (bits_model(s,d,le)==64 && scalar_model(s,d,317,1,le)==2
        && (scalar_model(s,d,259,1,le)==5 || scalar_model(s,d,259,1,le)==8 || scalar_model(s,d,259,1,le)==32946)))
    &&& alpha_field_valid(s,d,le)
    &&& ext_bits(s, d, ch, le)
    &&& (match lookup(s, d, 339, le) {
        None => bits_model(s,d,le)!=64, Some(t) => array_of(s, t, ch, if bits_model(s,d,le)==64 {3int} else {1int}, le),
    })
}

#[derive(Clone,Copy)]
struct ExtFormat {
    width:usize,height:usize,channels:usize,photo:usize,compression:usize,
    alpha:usize,depth:usize,fill:usize,output_len:usize,planar:usize,predictor:usize,
}
impl ExtFormat {
    spec fn matches(self,s:Seq<u8>,d:Directory,le:bool,limits:Limits) -> bool {
        &&& ext_fields(s,d,le,limits)
        &&& self.width==scalar_model(s,d,256,0,le)
        &&& self.height==scalar_model(s,d,257,0,le)
        &&& self.channels==scalar_model(s,d,277,1,le)
        &&& self.photo==scalar_model(s,d,262,999,le)
        &&& self.compression==scalar_model(s,d,259,1,le)
        &&& self.alpha==scalar_model(s,d,338,0,le)
        &&& self.alpha<=2
        &&& self.depth==bits_model(s,d,le)
        &&& self.planar==scalar_model(s,d,284,1,le)
        &&& self.fill==scalar_model(s,d,266,1,le)
        &&& self.predictor==scalar_model(s,d,317,1,le)
        &&& self.output_len as int==self.width as int*self.height*self.channels
    }
}
fn read_ext_format(s:&[u8],d:Directory,le:bool,limits:&Limits,budget:&mut Budget) -> (r:Result<ExtFormat,Error>)
    requires d.readable(s@,le),
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=12*(d.count as int)+4,
        r.is_ok() ==> r.unwrap().matches(s@,d,le,*limits),
        ext_fields(s@,d,le,*limits) && old(budget).remaining as int>=12*(d.count as int)+4 ==> r.is_ok(),
{
    let ghost eligible=ext_fields(s@,d,le,*limits) && budget.remaining as int>=12*(d.count as int)+4;
    reveal(ext_fields);
    let w = scalar(s, d, 256, 0, le, budget)?;
    let height = scalar(s, d, 257, 0, le, budget)?;
    if w == 0 || height == 0 { return Err(Error::Invalid); }
    if w > limits.max_dimension || height > limits.max_dimension { return Err(Error::Limit); }
    let channels = scalar(s, d, 277, 1, le, budget)?;
    if channels < 1 || channels > 4 { return Err(Error::Unsupported); }
    let photo = scalar(s, d, 262, 999, le, budget)?;
    if !((channels <= 2 && (photo == 0 || photo == 1)) || (channels >= 3 && photo == 2) || (channels == 1 && photo == 3)) {
        return Err(Error::Unsupported);
    }
    let compression = scalar(s, d, 259, 1, le, budget)?;
    if compression != 1 && compression != 32773 && compression != 5 && compression!=8 && compression!=32946 { return Err(Error::Unsupported); }
    let packbits = compression == 32773;
    let lzw = compression == 5;
    let planar=scalar(s,d,284,1,le,budget)?;
    let predictor=scalar(s,d,317,1,le,budget)?;
    if (planar!=1 && planar!=2) || scalar(s, d, 274, 1, le, budget)? != 1 {
        return Err(Error::Unsupported);
    }
    let alpha=read_alpha(s,d,le,budget)?;
    if !((alpha==0 && (channels==1 || channels==3)) || (alpha!=0 && (channels==2 || channels==4))) {
        return Err(Error::Unsupported);
    }
    let bits = find(s, d, 258, le, budget)?;
    let depth = match bits {
        None => 1,
        Some(t) => {
            if t.count != channels || t.kind != 3 { return Err(Error::Invalid); }
            proof { if eligible {
                array_entry(s@, t, channels as int, bits_model(s@, d, le), le, 0);
            } }
            value(s, t, 0, le)?
        }
    };
    if !((photo==3 && channels==1 && (depth==1 || depth==4 || depth==8)) || (photo!=3 && (depth==8 || ((depth==16 || (depth==64 && (photo==1 || photo==2))) && alpha==0) || (depth==1 && channels==1 && alpha==0)))) { return Err(Error::Unsupported); }
    if predictor!=1 && !(depth==64 && predictor==2 && (compression==5 || compression==8 || compression==32946)) {return Err(Error::Unsupported);}
    let sample_format=if depth==64 {3usize} else {1usize};
    let fill = scalar(s, d, 266, 1, le, budget)?;
    if fill != 1 { return Err(Error::Unsupported); }
    if (packbits || lzw) && fill != 1 { return Err(Error::Unsupported); }
    let formats = find(s, d, 339, le, budget)?;
    if depth==64 && formats.is_none() {return Err(Error::Unsupported);}
    if let Some(t) = formats {
        if t.count != channels || t.kind != 3 { return Err(Error::Invalid); }
    }
    assert(eligible ==> budget.remaining as int >=
        4);
    let ghost before_channels = budget.remaining;
    let mut c: usize = 0;
    while c < channels
        invariant c <= channels, budget.remaining <= old(budget).remaining,
            eligible == (ext_fields(s@,d,le,*limits) && old(budget).remaining as int>=12*(d.count as int)+4),
            eligible ==> before_channels as int >= 4,
            1 <= channels <= 4,
            (photo==3 && channels==1 && (depth==1 || depth==4 || depth==8)) || (photo!=3 && (depth==8 || ((depth==16 || (depth==64 && (photo==1 || photo==2))) && alpha==0) || (depth==1 && channels==1 && alpha==0))),
            depth == bits_model(s@, d, le),
            sample_format==if depth==64 {3usize} else {1usize},
            bits == lookup(s@, d, 258, le),
            match bits { None => depth == 1, Some(t) => t.count == channels && t.kind == 3 },
            eligible ==> match bits { None => true, Some(t) => array_of(s@, t, channels as int, depth as int, le) },
            eligible ==> match formats { None => true, Some(t) => array_of(s@, t, channels as int, sample_format as int, le) },
            budget.remaining as int + c == before_channels,
            old(budget).remaining as int-before_channels<=12*(d.count as int),
            match bits { None => true, Some(t) => forall|i: int| 0 <= i < c ==>
                #[trigger] value_valid(s@, t, i) },
            match bits { None => true, Some(t) => forall|i: int| 0 <= i < c ==>
                #[trigger] tag_value(s@, t, i, le) == depth },
            match formats { None => true, Some(t) => forall|i: int| 0 <= i < c ==> #[trigger] value_valid(s@,t,i) },
            match formats { None => true, Some(t) => forall|i: int| 0 <= i < c ==> #[trigger] tag_value(s@, t, i, le) == sample_format },
        decreases channels - c,
    {
        spend(budget, 1)?;
        if let Some(t) = bits {
            proof { if eligible { array_entry(s@, t, channels as int, depth as int, le, c as int); } }
            if value(s, t, c, le)? != depth { return Err(Error::Unsupported); }
        }
        if let Some(t) = formats {
            proof { if eligible { array_entry(s@, t, channels as int, sample_format as int, le, c as int); } }
            assert(eligible ==> value_valid(s@, t, c as int)
                && tag_value(s@, t, c as int, le) == sample_format);
            if value(s, t, c, le)? != sample_format { return Err(Error::Unsupported); }
        }
        c = c + 1;
    }
    proof {
        match bits {
            None => { assert(channels == 1); }
            Some(t) => {
                assert(t.kind == 3 && t.count == channels);
                assert forall|i: int| 0 <= i < channels implies
                    value_valid(s@, t, i) && #[trigger] tag_value(s@, t, i, le) == depth by {
                    assert(i < c);
                    assert(value_valid(s@, t, i));
                    assert(tag_value(s@, t, i, le) == depth);
                }
                assert(array_of(s@, t, channels as int, depth as int, le));
            }
        }
        assert(ext_bits(s@, d, channels as int, le));
    }
    let output_len = mul(mul(w, height)?, channels)?;
    if output_len > limits.max_output { return Err(Error::Limit); }
    proof {
        if let Some(t)=formats {
            assert(t.kind==3 && t.count==channels);
            assert forall|i:int| 0<=i<channels implies value_valid(s@,t,i)
                && #[trigger] tag_value(s@,t,i,le)==sample_format by {
                assert(i<c);
                assert(value_valid(s@,t,i));
                assert(tag_value(s@,t,i,le)==sample_format);
            }
            assert(array_of(s@,t,channels as int,sample_format as int,le));
        }
    }
    Ok(ExtFormat {width:w,height,channels,photo,compression,alpha,depth,fill,output_len,planar,predictor})
}



spec fn plane_ready(s:Seq<u8>,g:Geometry,len:int) -> bool {
    &&& g.positive() && g.alpha==0 && !g.white && !g.bilevel && !g.lsb
    &&& len==g.width as int*g.height*g.channels && 0<=len<=usize::MAX
    &&& g.across as int==(g.width as int-1)/(g.block_width as int)+1
    &&& segment_count(g)<=usize::MAX
    &&& g.offsets.count==segment_count(g) && g.counts.count==segment_count(g)
    &&& forall|j:int| 0<=j<segment_count(g) ==> segment_valid(s,g,g.tiled,j)
    &&& forall|k:int| 0<=k<len ==> sample_valid(s,g,k)
}

#[verifier::opaque]
spec fn plane_profile(s:Seq<u8>,g:Geometry) -> bool {
    &&& 0<g.row_bytes()<=usize::MAX
    &&& forall|j:int| 0<=j<segment_count(g) ==> segment_valid(s,g,g.tiled,j)
    &&& 0<=codec_work_prefix(s,g,segment_count(g) as nat)<=usize::MAX
}

fn validate_plane(s:&[u8],g:Geometry,len:usize,budget:&mut Budget) -> (r:Result<usize,Error>)
    requires g.positive(),g.alpha==0,!g.white,!g.bilevel,!g.lsb,
        len as int==g.width as int*g.height*g.channels,
        g.across as int==(g.width as int-1)/(g.block_width as int)+1,
        !g.tiled ==> g.block_width==g.width,
        segment_count(g)<=usize::MAX,
        g.offsets.count==segment_count(g),g.counts.count==segment_count(g),
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=segment_count(g)*(s.len() as int+1),
        r.is_ok() ==> plane_ready(s@,g,len as int),
        r.is_ok() ==> r.unwrap() as int==codec_work_prefix(s@,g,segment_count(g) as nat),
        plane_profile(s@,g) && old(budget).remaining as int>=segment_count(g)*(s.len() as int+1) ==> r.is_ok(),
{
    let ghost initial=budget.remaining;
    let ghost eligible=plane_profile(s@,g) && budget.remaining as int>=segment_count(g)*(s.len() as int+1);
    reveal(plane_profile);
    let stride=mul(g.block_width,g.channels)?;
    assert(stride>0) by(nonlinear_arith) requires g.block_width>0,g.channels>0,stride as int==g.block_width as int*g.channels;
    let mut j=0usize;
    let mut work=0usize;
    proof {codec_work_zero(s@,g);}
    while j<g.counts.count
        invariant initial==old(budget).remaining, eligible==(plane_profile(s@,g) && old(budget).remaining as int>=segment_count(g)*(s.len() as int+1)),
            j<=g.counts.count, g.positive(),g.alpha==0,!g.white,!g.bilevel,!g.lsb,
            len as int==g.width as int*g.height*g.channels,
            g.across as int==(g.width as int-1)/(g.block_width as int)+1,
            !g.tiled ==> g.block_width==g.width,
            g.offsets.count==segment_count(g),g.counts.count==segment_count(g),
            stride as int==g.row_bytes(),stride>0,
            budget.remaining<=old(budget).remaining,
            old(budget).remaining as int-budget.remaining<=j as int*(s.len() as int+1),
            work as int==codec_work_prefix(s@,g,j as nat),
            forall|q:int| 0<=q<j ==> segment_valid(s@,g,g.tiled,q),
            eligible ==> forall|q:int| 0<=q<g.counts.count ==> segment_valid(s@,g,g.tiled,q),
            eligible ==> 0<=codec_work_prefix(s@,g,segment_count(g) as nat)<=usize::MAX,
            eligible ==> old(budget).remaining as int>=segment_count(g)*(s.len() as int+1),
        decreases g.counts.count-j,
    {
        assert(eligible ==> segment_valid(s@,g,g.tiled,j as int));
        reveal(segment_valid);
        proof {if eligible {traversal_budget(old(budget).remaining as int,budget.remaining as int,j as int,g.counts.count as int-1,s.len() as int,0);}}
        let ghost before=budget.remaining;
        assert((j as int+1)*(s.len() as int+1)<=segment_count(g)*(s.len() as int+1)) by(nonlinear_arith) requires j<g.counts.count,g.counts.count==segment_count(g),s.len()>=0;
        assert(initial as int-budget.remaining+1<=segment_count(g)*(s.len() as int+1)) by(nonlinear_arith) requires initial as int-budget.remaining<=j as int*(s.len() as int+1),j<g.counts.count,g.counts.count==segment_count(g),s.len()>=0;
        spend(budget,1)?;
        let start=value(s,g.offsets,j,g.le)?;
        let count=value(s,g.counts,j,g.le)?;
        if !checked_range(s.len(),start,count) {return Err(Error::Truncated);}
        let rows=if g.tiled {g.block_height} else {
            let first=mul(j,g.block_height)?;
            if first>=g.height {return Err(Error::Invalid);}
            if g.height-first<g.block_height {g.height-first} else {g.block_height}
        };
        let needed=mul(mul(g.block_width,rows)?,g.channels)?;
        assert(initial as int-budget.remaining+count<=segment_count(g)*(s.len() as int+1)) by(nonlinear_arith) requires initial as int-before<=j as int*(s.len() as int+1),before as int==budget.remaining+1,0<=count<=s.len(),j<g.counts.count,g.counts.count==segment_count(g);
        if g.packbits {pb_validate(s,start,count,stride,needed,budget)?;}
        else if g.lzw {lz_validate(s,start,count,needed,budget)?;}
        else if count<needed {return Err(Error::Truncated);}
        assert(segment_valid(s@,g,g.tiled,j as int));
        proof {
            if eligible {codec_work_monotone(s@,g,(j+1) as nat,segment_count(g) as nat);}
            codec_work_step(s@,g,j as nat);
        }
        // Includes storage and source for all codecs, so conversion's quota is uniform.
        work=add(add(work,count)?,needed)?;
        assert(before as int-budget.remaining<=s.len() as int+1);
        assert((j as int+1)*(s.len() as int+1)==j as int*(s.len() as int+1)+s.len()+1) by(nonlinear_arith);
        j=j+1;
    }
    proof {assert forall|k:int| 0<=k<len implies sample_valid(s@,g,k) by {segment_covers_sample(s@,g,g.tiled,k);}}
    Ok(work)
}

    fn plane_decode_compressed(s:&[u8],g:Geometry,plen:usize, output: &mut [u8]) -> (r: Result<(), Error>)
        requires plane_ready(s@,g,plen as int), (g.packbits || g.lzw),
            old(output).len() >= plen,
        ensures r.is_ok(), final(output).len() == old(output).len(),
            forall|k: int| 0 <= k < plen ==> final(output)@[k] == storage_pixel(s@,g,k),
            forall|k: int| plen <= k < old(output).len() ==> final(output)@[k] == old(output)@[k],
    {
        let mut j=0usize;
        while j<g.counts.count
            invariant plane_ready(s@,g,plen as int), g == g, (g.packbits || g.lzw), j<=g.counts.count,
                output.len() == old(output).len(), output.len() >= plen,
                forall|k: int| 0 <= k < plen ==> output@[k] ==
                    (if g.segment(k) < j { storage_pixel(s@,g,k) } else { old(output)@[k] }),
                forall|k: int| plen <= k < old(output).len() ==> output@[k] == old(output)@[k],
            decreases g.counts.count-j,
        {
            assert(segment_valid(s@,g,g.tiled,j as int));
            if g.lzw { decode_lz_segment(s,g,j,output)?; }
            else { decode_pb_segment(s,g,j,output)?; }
            j=j+1;
        }
        proof {
            assert forall|k: int| 0 <= k < plen implies
                0 <= #[trigger] g.segment(k) < g.counts.count by {
                assert(sample_valid(s@,g,k));
            }
        }
        Ok(())
    }

    fn plane_decode_plain(s:&[u8],g:Geometry,plen:usize,output:&mut [u8]) -> (r:Result<(),Error>)
        requires plane_ready(s@,g,plen as int), !g.packbits && !g.lzw,
            old(output).len()>=plen,
        ensures r.is_ok(), final(output).len()==old(output).len(),
            forall|i:int| 0<=i<plen ==> final(output)@[i]==storage_pixel(s@,g,i),
            forall|i:int| plen<=i<old(output).len() ==> final(output)@[i]==old(output)@[i],
    {
        let mut k=0usize;
        while k<plen
            invariant plane_ready(s@,g,plen as int), !g.packbits && !g.lzw,
                k<=plen<=output.len(), output.len()==old(output).len(),
                forall|i:int| 0<=i<k ==> output@[i]==storage_pixel(s@,g,i),
                forall|i:int| plen<=i<output.len() ==> output@[i]==old(output)@[i],
            decreases plen-k,
        {
            output[k]=sample(s,g,k)?;
            k=k+1;
        }
        Ok(())
    }


}
