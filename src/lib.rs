#![forbid(unsafe_code)]
mod vp8_bool;
mod vp8_tables;
mod vp8_fields;
mod vp8_header;
mod vp8_tree;
mod vp8_coeff;
mod vp8_transform;
mod vp8_predict;
mod vp8_modes;
mod vp8_quant;
mod vp8_residual;
mod vp8_edges;
mod vp8_render;
mod vp8_filter_kernel;
mod vp8_plane;
mod vp8_filter_info;
mod vp8_filter_write;
mod vp8_frame;
mod vp8_filter_plane;
mod vp8_macroblock;
mod vp8_image;
mod vp8_postfilter;
mod vp8_color;
pub mod native_vp8;
use vstd::prelude::*;

mod lerc_numeric;
mod lerc_checksum;
mod lerc_bits;
mod lerc_word;
mod lerc_mask;
mod lerc_header;
mod lerc_samples;
mod lerc_block;
mod lerc_image;
mod lerc_huffman;
mod lerc_byte_image;
mod lerc_planes;
pub mod native_lerc;
mod zstd_bits;
mod zstd_fse;
mod zstd_table;
mod zstd_weights;
mod zstd_huffman;
mod zstd_description;
mod zstd_literals;
mod zstd_sequence_tables;
mod zstd_sequence_token;
mod zstd_output;
mod zstd_sequences;
mod zstd_checksum;
mod zstd_frame;
pub mod native_zstd;
pub mod native;
pub mod native_samples;
pub mod native_rgba;
pub mod native_fp;
pub mod native_stream;
pub mod native_rle;
pub mod container;
pub mod adapter_meta;
pub mod adapter_bounds;
pub mod adapter_geometry;
pub mod adapter_window;
pub mod adapter_color;

mod logluv;
mod logluv_format;
mod logluv_segment;
mod logluv_tiff;
mod logluv_witness;
pub use logluv_tiff::*;
mod webp_pixels;
mod webp_predict;
mod webp_riff;
mod webp_alpha;
mod webp_aux;
mod webp_transforms;
mod webp_inverse;
mod webp_simd;
mod webp_spans;
mod webp_prepare;
pub mod native_webp;
mod webp_huffman;
mod webp_header;
mod webp_container;
mod webp_token;
mod webp;
mod webp_format;
mod webp_segment;
mod webp_tiff;
mod webp_witness;
pub use webp_tiff::*;
mod fax_tables;
mod fax_bits;
mod fax_row;
mod fax_frame;
mod fax;
mod fax_meta;
mod fax_format;
mod fax_segment;
mod fax_tiff;
mod fax_witness;
pub use fax_tiff::*;
mod bit_window;
mod jpeg_bits;
mod jpeg_huffman;
mod jpeg_transform;
mod jpeg_block;
mod jpeg_header;
mod jpeg_scan;
mod jpeg_color;
mod jpeg_layout;
mod jpeg;
mod jpeg_meta;
mod jpeg_format;
mod jpeg_container;
mod jpeg_segment;
mod jpeg_tiff;
mod jpeg_witness;
pub use jpeg_tiff::*;
pub use jpeg::*;
mod deflate_bits;
mod deflate_header;
mod deflate;
mod deflate_plane;
mod extended;
mod raster_layout;
mod float64;
mod reader;
pub use reader::*;
pub use extended::*;

verus! {

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    Truncated,
    Invalid,
    Unsupported,
    Limit,
    OutputTooSmall,
    PageNotFound,
}

pub open spec fn range_ok(len: int, off: int, size: int) -> bool {
    0 <= off && 0 <= size && off + size <= len
}

pub fn checked_range(len: usize, off: usize, size: usize) -> (r: bool)
    ensures r == range_ok(len as int, off as int, size as int),
{
    off <= len && size <= len - off
}

pub open spec fn word16(s: Seq<u8>, off: int, le: bool) -> int {
    if le { s[off] as int + 256 * s[off + 1] as int }
    else { 256 * s[off] as int + s[off + 1] as int }
}

pub fn read16(s: &[u8], off: usize, le: bool) -> (r: Result<u16, Error>)
    ensures
        r.is_ok() == range_ok(s.len() as int, off as int, 2),
        r.is_ok() ==> r.unwrap() as int == word16(s@, off as int, le),
{
    if !checked_range(s.len(), off, 2) { return Err(Error::Truncated); }
    let a = s[off] as u16;
    let b = s[off + 1] as u16;
    if le { Ok(a + b * 256) } else { Ok(a * 256 + b) }
}

pub open spec fn word32(s: Seq<u8>, off: int, le: bool) -> int {
    if le { word16(s, off, le) + 65536 * word16(s, off + 2, le) }
    else { 65536 * word16(s, off, le) + word16(s, off + 2, le) }
}

pub fn read32(s: &[u8], off: usize, le: bool) -> (r: Result<u32, Error>)
    ensures
        r.is_ok() == range_ok(s.len() as int, off as int, 4),
        r.is_ok() ==> r.unwrap() as int == word32(s@, off as int, le),
{
    if !checked_range(s.len(), off, 4) { return Err(Error::Truncated); }
    let a = match read16(s, off, le) { Ok(v) => v as u32, Err(e) => return Err(e) };
    let b = match read16(s, off + 2, le) { Ok(v) => v as u32, Err(e) => return Err(e) };
    if le { Ok(a + b * 65536) } else { Ok(a * 65536 + b) }
}

pub fn add(a: usize, b: usize) -> (r: Result<usize, Error>)
    ensures r.is_ok() == (a as int + b as int <= usize::MAX),
        r.is_ok() ==> r.unwrap() == a + b,
{
    match a.checked_add(b) { Some(v) => Ok(v), None => Err(Error::Limit) }
}

pub fn mul(a: usize, b: usize) -> (r: Result<usize, Error>)
    ensures r.is_ok() == (a as int * b as int <= usize::MAX),
        r.is_ok() ==> r.unwrap() == a * b,
{
    match a.checked_mul(b) { Some(v) => Ok(v), None => Err(Error::Limit) }
}

#[derive(Debug, Clone, Copy)]
pub struct Budget { pub remaining: usize }

pub fn spend(b: &mut Budget, n: usize) -> (r: Result<(), Error>)
    ensures final(b).remaining <= old(b).remaining,
        r.is_ok() == (n <= old(b).remaining),
        r.is_ok() ==> final(b).remaining as int + n == old(b).remaining,
        r.is_err() ==> final(b).remaining == old(b).remaining,
{
    if n > b.remaining { return Err(Error::Limit); }
    b.remaining = b.remaining - n;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_input: usize,
    pub max_dimension: usize,
    pub max_output: usize,
    pub max_directories: usize,
    pub max_tags: usize,
    pub max_segments: usize,
}

impl Limits {
    pub fn conservative() -> Self {
        Self { max_input: 67108864, max_dimension: 65536, max_output: 67108864,
            max_directories: 1024, max_tags: 4096, max_segments: 1048576 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Header { pub little_endian: bool, pub first_ifd: usize }

pub open spec fn header_valid(s: Seq<u8>) -> bool {
    s.len() >= 8
    && ((s[0] == 73 && s[1] == 73) || (s[0] == 77 && s[1] == 77))
    && word16(s, 2, s[0] == 73) == 42
}

pub fn header(s: &[u8]) -> (r: Result<Header, Error>)
    ensures r.is_ok() == header_valid(s@),
        r.is_ok() ==> s.len() >= 8,
        r.is_ok() ==> word16(s@, 2, r.unwrap().little_endian) == 42,
        r.is_ok() ==> r.unwrap().first_ifd == word32(s@, 4, r.unwrap().little_endian),
        r.is_ok() ==> (if r.unwrap().little_endian {
            s@[0] == 73 && s@[1] == 73
        } else { s@[0] == 77 && s@[1] == 77 }),
{
    if s.len() < 8 { return Err(Error::Truncated); }
    let le = if s[0] == 73 && s[1] == 73 { true }
        else if s[0] == 77 && s[1] == 77 { false }
        else { return Err(Error::Invalid); };
    let magic = read16(s, 2, le)?;
    if magic == 43 { return Err(Error::Unsupported); }
    if magic != 42 { return Err(Error::Invalid); }
    Ok(Header { little_endian: le, first_ifd: read32(s, 4, le)? as usize })
}

#[derive(Debug, Clone, Copy)]
struct Tag { kind: u16, count: usize, data: usize }

pub open spec fn field_size(k: u16) -> int {
    if k == 1 || k == 2 || k == 6 || k == 7 { 1 }
    else if k == 3 || k == 8 { 2 }
    else if k == 4 || k == 9 || k == 11 || k == 13 { 4 }
    else if k == 5 || k == 10 || k == 12 { 8 }
    else { 0 }
}

fn size_of_field(k: u16) -> (v: usize)
    ensures v == field_size(k), v <= 8,
{
    if k == 1 || k == 2 || k == 6 || k == 7 { 1 }
    else if k == 3 || k == 8 { 2 }
    else if k == 4 || k == 9 || k == 11 || k == 13 { 4 }
    else if k == 5 || k == 10 || k == 12 { 8 }
    else { 0 }
}

#[verifier::opaque]
spec fn tag_valid(s: Seq<u8>, off: int, le: bool) -> bool {
    let kind = word16(s, off + 2, le) as u16;
    let count = word32(s, off + 4, le);
    let size = count * field_size(kind);
    let data = if size <= 4 { off + 8 } else { word32(s, off + 8, le) };
    &&& range_ok(s.len() as int, off, 12)
    &&& 0 <= count <= usize::MAX
    &&& field_size(kind) > 0
    &&& 0 <= size <= usize::MAX
    &&& 0 <= data <= usize::MAX
    &&& range_ok(s.len() as int, data, size)
}

fn tag_at(s: &[u8], off: usize, le: bool) -> (r: Result<Tag, Error>)
    ensures r.is_ok() == tag_valid(s@, off as int, le),
        r.is_ok() ==> range_ok(s.len() as int, r.unwrap().data as int,
        r.unwrap().count as int * field_size(r.unwrap().kind)),
        r.is_ok() ==> field_size(r.unwrap().kind) > 0,
        r.is_ok() ==> range_ok(s.len() as int, off as int, 12),
        r.is_ok() ==> r.unwrap().kind == word16(s@, off as int + 2, le),
        r.is_ok() ==> r.unwrap().count == word32(s@, off as int + 4, le),
        r.is_ok() ==> r.unwrap().data == (if r.unwrap().count as int * field_size(r.unwrap().kind) <= 4 {
            off as int + 8
        } else { word32(s@, off as int + 8, le) }),
{
    reveal(tag_valid);
    if !checked_range(s.len(), off, 12) { return Err(Error::Truncated); }
    let kind = read16(s, off + 2, le)?;
    let count = read32(s, off + 4, le)? as usize;
    let size = size_of_field(kind);
    if size == 0 { return Err(Error::Unsupported); }
    let bytes = mul(count, size)?;
    let data = if bytes <= 4 { off + 8 } else { read32(s, off + 8, le)? as usize };
    if !checked_range(s.len(), data, bytes) { return Err(Error::Truncated); }
    Ok(Tag { kind, count, data })
}

spec fn value_valid(s: Seq<u8>, t: Tag, i: int) -> bool {
    0 <= i < t.count && (t.kind == 3 || t.kind == 4)
    && i * (if t.kind == 3 { 2int } else { 4int }) <= usize::MAX
    && t.data as int + i * (if t.kind == 3 { 2int } else { 4int }) <= usize::MAX
    && range_ok(s.len() as int, t.data as int + i * (if t.kind == 3 { 2int } else { 4int }),
        if t.kind == 3 { 2 } else { 4 })
}

fn value(s: &[u8], t: Tag, i: usize, le: bool) -> (r: Result<usize, Error>)
    ensures r.is_ok() == value_valid(s@, t, i as int),
        r.is_ok() ==> i < t.count,
        r.is_ok() ==> t.kind == 3 || t.kind == 4,
        r.is_ok() ==> r.unwrap() == (if t.kind == 3 {
            word16(s@, t.data as int + i as int * 2, le)
        } else { word32(s@, t.data as int + i as int * 4, le) }),
{
    assert(s@.len() == s.len());
    reveal(value_valid);
    if i >= t.count { return Err(Error::Invalid); }
    if t.kind == 3 { Ok(read16(s, add(t.data, mul(i, 2)?)?, le)? as usize) }
    else if t.kind == 4 { Ok(read32(s, add(t.data, mul(i, 4)?)?, le)? as usize) }
    else { Err(Error::Invalid) }
}

#[derive(Debug, Clone, Copy)]
struct Directory { start: usize, count: usize, next: usize }

impl Directory {
    spec fn readable(self, s: Seq<u8>, le: bool) -> bool {
        range_ok(s.len() as int, self.start as int, self.count as int * 12 + 4)
        && forall|i: int| 0 <= i < self.count ==> #[trigger] tag_valid(s, self.start as int + i * 12, le)
    }
}

spec fn directory_valid(s: Seq<u8>, off: int, le: bool, limits: Limits) -> bool {
    let n = word16(s, off, le);
    &&& range_ok(s.len() as int, off, 2)
    &&& 0 <= n <= limits.max_tags
    &&& 0 <= off + 2 <= usize::MAX
    &&& 0 <= n * 12 + 4 <= usize::MAX
    &&& range_ok(s.len() as int, off + 2, n * 12 + 4)
    &&& (forall|i: int| 0 <= i < n ==> #[trigger] tag_valid(s, off + 2 + i * 12, le))
    &&& (forall|i: int| 1 <= i < n ==>
        #[trigger] word16(s, off + 2 + (i - 1) * 12, le) < #[trigger] word16(s, off + 2 + i * 12, le))
}

spec fn next_ifd(s: Seq<u8>, off: int, le: bool) -> int {
    word32(s, off + 2 + word16(s, off, le) * 12, le)
}

spec fn nth_ifd(s: Seq<u8>, first: int, le: bool, page: nat) -> int
    decreases page,
{
    if page == 0 { first }
    else { next_ifd(s, nth_ifd(s, first, le, (page - 1) as nat), le) }
}

spec fn directory_model(s: Seq<u8>, off: int, le: bool) -> Directory {
    Directory { start: (off + 2) as usize, count: word16(s, off, le) as usize,
        next: next_ifd(s, off, le) as usize }
}

fn directory(s: &[u8], off: usize, le: bool, limits: &Limits,
    budget: &mut Budget) -> (r: Result<Directory, Error>)
    ensures final(budget).remaining <= old(budget).remaining,
        old(budget).remaining as int - final(budget).remaining <= 1 + limits.max_tags,
        r.is_ok() ==> r.unwrap() == directory_model(s@, off as int, le),
        r.is_ok() ==> r.unwrap().readable(s@, le),
        directory_valid(s@, off as int, le, *limits) && old(budget).remaining as int >= 1 + word16(s@, off as int, le) ==> r.is_ok(),
        r.is_ok() ==> range_ok(s.len() as int, r.unwrap().start as int,
            r.unwrap().count as int * 12 + 4),
        r.is_ok() ==> r.unwrap().count <= limits.max_tags,
        r.is_ok() ==> r.unwrap().start == off as int + 2,
        r.is_ok() ==> r.unwrap().count == word16(s@, off as int, le),
        r.is_ok() ==> r.unwrap().next == next_ifd(s@, off as int, le),
{
    spend(budget, 1)?;
    let count = read16(s, off, le)? as usize;
    if count > limits.max_tags { return Err(Error::Limit); }
    let start = add(off, 2)?;
    let bytes = mul(count, 12)?;
    let size = add(bytes, 4)?;
    if !checked_range(s.len(), start, size) { return Err(Error::Truncated); }
    let mut i: usize = 0;
    let mut cursor = start;
    let mut previous: u16 = 0;
    while i < count
        invariant i <= count, count <= limits.max_tags,
            cursor as int == start as int + i as int * 12,
            count == word16(s@, off as int, le), start as int == off as int + 2,
            range_ok(s.len() as int, start as int, count as int * 12 + 4),
            budget.remaining as int + i + 1 == old(budget).remaining,
            i > 0 ==> previous == word16(s@, start as int + (i as int - 1) * 12, le),
            forall|j: int| 0 <= j < i ==> #[trigger] tag_valid(s@, start as int + j * 12, le),
        decreases count - i,
    {
        spend(budget, 1)?;
        let id = read16(s, cursor, le)?;
        if i > 0 && id <= previous { return Err(Error::Invalid); }
        tag_at(s, cursor, le)?;
        previous = id;
        i = i + 1;
        cursor = cursor + 12;
    }
    let next = read32(s, cursor, le)? as usize;
    Ok(Directory { start, count, next })
}

spec fn tag_model(s: Seq<u8>, pos: int, le: bool) -> Tag {
    let kind = word16(s, pos + 2, le) as u16;
    let count = word32(s, pos + 4, le) as usize;
    Tag { kind, count, data: (if count as int * field_size(kind) <= 4 {
        pos + 8
    } else { word32(s, pos + 8, le) }) as usize }
}

spec fn first_match(s: Seq<u8>, d: Directory, id: u16, le: bool, j: int) -> bool {
    0 <= j < d.count && word16(s, d.start as int + j * 12, le) == id
    && forall|k: int| 0 <= k < j ==> #[trigger] word16(s, d.start as int + k * 12, le) != id
}

#[verifier::opaque]
spec fn lookup(s: Seq<u8>, d: Directory, id: u16, le: bool) -> Option<Tag> {
    if exists|j: int| first_match(s, d, id, le, j) {
        let j = choose|j: int| first_match(s, d, id, le, j);
        Some(tag_model(s, d.start as int + j * 12, le))
    } else { None }
}

proof fn lookup_at(s: Seq<u8>, d: Directory, id: u16, le: bool, i: int)
    requires first_match(s, d, id, le, i),
    ensures lookup(s, d, id, le) == Some(tag_model(s, d.start as int + i * 12, le)),
{
    reveal(lookup);
    let j = choose|j: int| first_match(s, d, id, le, j);
    if j < i { assert(word16(s, d.start as int + j * 12, le) != id); }
    if i < j { assert(word16(s, d.start as int + i * 12, le) != id); }
    assert(i == j);
}

proof fn lookup_missing(s: Seq<u8>, d: Directory, id: u16, le: bool)
    requires forall|i: int| 0 <= i < d.count ==>
        #[trigger] word16(s, d.start as int + i * 12, le) != id,
    ensures lookup(s, d, id, le).is_none(),
{
    reveal(lookup);
}

spec fn scalar_model(s: Seq<u8>, d: Directory, id: u16, default: usize, le: bool) -> int {
    match lookup(s, d, id, le) { Some(t) => tag_value(s, t, 0, le), None => default as int }
}

fn find(s: &[u8], d: Directory, id: u16, le: bool,
    budget: &mut Budget) -> (r: Result<Option<Tag>, Error>)
    ensures final(budget).remaining <= old(budget).remaining,
        old(budget).remaining as int - final(budget).remaining <= d.count,
        r.is_ok() ==> r.unwrap() == lookup(s@, d, id, le),
        d.readable(s@, le) && old(budget).remaining >= d.count ==> r.is_ok(),
{
    let mut i: usize = 0;
    while i < d.count
        invariant i <= d.count, budget.remaining as int + i == old(budget).remaining,
            s@.len() == s.len(), s@.len() <= usize::MAX,
            forall|j: int| 0 <= j < i ==> #[trigger] word16(s@, d.start as int + j * 12, le) != id,
        decreases d.count - i,
    {
        spend(budget, 1)?;
        let pos = add(d.start, mul(i, 12)?)?;
        let key = read16(s, pos, le)?;
        if key == id {
            let tag = tag_at(s, pos, le)?;
            proof { lookup_at(s@, d, id, le, i as int); }
            return Ok(Some(tag));
        }
        i = i + 1;
    }
    reveal(lookup);
    Ok(None)
}

spec fn scalar_valid(s: Seq<u8>, d: Directory, id: u16, le: bool) -> bool {
    match lookup(s, d, id, le) {
        None => true,
        Some(t) => t.count == 1 && (t.kind == 3 ||
            ((id == 256 || id == 257 || id == 278 || id == 322 || id == 323) && t.kind == 4))
            && value_valid(s, t, 0),
    }
}

fn scalar(s: &[u8], d: Directory, id: u16, default: usize, le: bool,
    budget: &mut Budget) -> (r: Result<usize, Error>)
    ensures final(budget).remaining <= old(budget).remaining,
        old(budget).remaining as int - final(budget).remaining <= d.count,
        r.is_ok() ==> r.unwrap() == scalar_model(s@, d, id, default, le),
        r.is_ok() ==> scalar_valid(s@, d, id, le),
        d.readable(s@, le) && scalar_valid(s@, d, id, le) && old(budget).remaining >= d.count ==> r.is_ok(),
{
    match find(s, d, id, le, budget)? {
        Some(t) => {
            let wide = id == 256 || id == 257 || id == 278 || id == 322 || id == 323;
            if t.count != 1 || (t.kind != 3 && !(wide && t.kind == 4)) { return Err(Error::Invalid); }
            value(s, t, 0, le)
        }
        None => Ok(default),
    }
}

fn required_tag(s: &[u8], d: Directory, id: u16, le: bool,
    budget: &mut Budget) -> (r: Result<Tag, Error>)
    ensures final(budget).remaining <= old(budget).remaining,
        old(budget).remaining as int - final(budget).remaining <= d.count,
        r.is_ok() ==> lookup(s@, d, id, le) == Some(r.unwrap()),
        d.readable(s@, le) && lookup(s@, d, id, le).is_some() && old(budget).remaining >= d.count ==> r.is_ok(),
{
    match find(s, d, id, le, budget)? { Some(t) => Ok(t), None => Err(Error::Invalid) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaMode { None, Associated, Unassociated }

#[verifier::opaque]
spec fn alpha_field_valid(s:Seq<u8>,d:Directory,le:bool) -> bool {
    match lookup(s,d,338,le) {
        None=>true,
        Some(t)=> t.kind==3 && t.count==1 && value_valid(s,t,0)
            && (tag_value(s,t,0,le)==1 || tag_value(s,t,0,le)==2),
    }
}
fn read_alpha(s:&[u8],d:Directory,le:bool,budget:&mut Budget) -> (r:Result<usize,Error>)
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=d.count,
        r.is_ok() ==> r.unwrap() == scalar_model(s@,d,338,0,le),
        r.is_ok() ==> r.unwrap()<=2 && alpha_field_valid(s@,d,le),
        d.readable(s@,le) && alpha_field_valid(s@,d,le) && old(budget).remaining>=d.count ==> r.is_ok(),
{
    reveal(alpha_field_valid);
    match find(s,d,338,le,budget)? {
        None=>Ok(0),
        Some(t)=>{
            if t.kind!=3 || t.count!=1 {return Err(Error::Invalid);}
            let mode=value(s,t,0,le)?;
            if mode!=1 && mode!=2 {return Err(Error::Unsupported);}
            Ok(mode)
        },
    }
}

// Output uses straight alpha. Unassociation rounds to nearest and clips
// out-of-range color to the representable 8-bit range. At zero associated
// alpha, TIFF says stored color is ignored; the canonical output color is 0.
spec fn straight_color(v:u8,a:u8,associated:bool,white:bool) -> u8 {
    if associated && a==0 {0u8} else {
        let q=if associated {(v as int*255+(a as int)/2)/(a as int)} else {v as int};
        let c=if q>255 {255int} else {q};
        (if white {255-c} else {c}) as u8
    }
}
fn convert_color(v:u8,a:u8,associated:bool,white:bool) -> (r:u8)
    ensures r == straight_color(v,a,associated,white),
{
    if associated && a==0 {return 0;}
    let q=if associated { (v as u32*255+(a as u32)/2)/(a as u32) } else {v as u32};
    let c=if q>255 {255u8} else {q as u8};
    if white {255-c} else {c}
}

spec fn alpha_index(g:Geometry,k:int) -> int {
    k-k%(g.channels as int)+g.channels-1
}
spec fn output_pixel(s:Seq<u8>,g:Geometry,k:int) -> u8 {
    if g.alpha==0 || k%(g.channels as int)==g.channels-1 {storage_pixel(s,g,k)}
    else {straight_color(storage_pixel(s,g,k),storage_pixel(s,g,alpha_index(g,k)),g.alpha==1,g.white)}
}
spec fn decode_base(g:Geometry) -> int {
    let n=g.width as int*g.height*g.channels;
    n+if g.alpha!=0 {n} else {0int}
}
proof fn alpha_index_bounds(g:Geometry,k:int)
    requires g.positive(), 0<=k<g.width as int*g.height*g.channels,
    ensures k<=alpha_index(g,k)<g.width as int*g.height*g.channels,
        0<=k%(g.channels as int)<g.channels,
        k%(g.channels as int)<=k,
{
    let ch=g.channels as int;
    let n=g.width as int*g.height;
    quotient_remainder(k,ch);
    assert(k/ch<n) by(nonlinear_arith)
        requires k< n*ch, k==ch*(k/ch)+k%ch, ch>0, k%ch>=0;
    assert(alpha_index(g,k)<n*ch) by(nonlinear_arith)
        requires k==ch*(k/ch)+k%ch, k/ch<n, ch>0,
            alpha_index(g,k)==k-k%ch+ch-1;
}
fn convert_alpha(_s:&[u8],g:Geometry,n:usize,output:&mut [u8])
    requires g.positive(), g.alpha!=0, n as int==g.width as int*g.height*g.channels,
        n<=old(output).len(),
        forall|i:int| 0<=i<n ==> old(output)@[i]==storage_pixel(_s@,g,i),
    ensures final(output).len()==old(output).len(),
        forall|i:int| 0<=i<n ==> final(output)@[i]==output_pixel(_s@,g,i),
        forall|i:int| n<=i<old(output).len() ==> final(output)@[i]==old(output)@[i],
{
    // Straight, non-inverted samples already have the requested representation.
    // Keep the conservative public quota; avoid a redundant image traversal.
    if g.alpha!=1 && !g.white {
        assert forall|i:int| 0<=i<n implies output@[i]==output_pixel(_s@,g,i) by {
            alpha_index_bounds(g,i);
        }
        return;
    }
    let mut k=0usize;
    while k<n
        invariant g.positive(), g.alpha!=0, n as int==g.width as int*g.height*g.channels,
            k<=n<=output.len(), output.len()==old(output).len(),
            forall|i:int| 0<=i<n ==> #[trigger] output@[i]==
                (if i<k {output_pixel(_s@,g,i)} else {storage_pixel(_s@,g,i)}),
            forall|i:int| n<=i<old(output).len() ==> output@[i]==old(output)@[i],
        decreases n-k,
    {
        proof {alpha_index_bounds(g,k as int);}
        let ai=(k-k%g.channels)+(g.channels-1);
        assert(output@[ai as int]==storage_pixel(_s@,g,alpha_index(g,k as int)));
        let v=output[k];
        let a=output[ai];
        let value=if k==ai {v} else {convert_color(v,a,g.alpha==1,g.white)};
        assert(value==output_pixel(_s@,g,k as int));
        let ghost prior_output=output@;
        let ghost prior_k=k;
        output[k]=value;
        k=k+1;
        assert forall|i:int| 0<=i<n implies #[trigger] output@[i]==
            (if i<k {output_pixel(_s@,g,i)} else {storage_pixel(_s@,g,i)}) by {
            if i!=prior_k {assert(prior_output[i]==(if i<prior_k {output_pixel(_s@,g,i)} else {storage_pixel(_s@,g,i)}));}
        }
    }
}

// Modern TIFF LZW and old libtiff LSB-packed LZW share a bounded dictionary.
spec fn lz_initial() -> Seq<Seq<u8>> {
    Seq::new(258, |i: int| if i < 256 { seq![i as u8] } else { Seq::empty() })
}
spec fn lz_width(n: int, legacy: bool) -> int {
    if legacy {
        if n < 512 { 9 } else if n < 1024 { 10 } else if n < 2048 { 11 } else { 12 }
    } else {
        if n < 511 { 9 } else if n < 1023 { 10 } else if n < 2047 { 11 } else { 12 }
    }
}
spec fn lz_defined(d: Seq<Seq<u8>>, c: int) -> bool {
    0 <= c < 256 || 258 <= c < d.len()
}
spec fn lz_data_ok(d: Seq<Seq<u8>>, prev: int, c: int, legacy: bool) -> bool {
    (prev == 256 || d.len() < 4096 || legacy)
    && (lz_defined(d,c) || (c == d.len() && lz_defined(d,prev)))
}
spec fn lz_phrase(d: Seq<Seq<u8>>, prev: int, c: int) -> Seq<u8> {
    if c == d.len() { d[prev].push(d[prev][0]) } else { d[c] }
}
spec fn lz_after(d: Seq<Seq<u8>>, prev: int, c: int) -> Seq<Seq<u8>> {
    if prev == 256 || d.len() == 4096 { d } else { d.push(d[prev].push(lz_phrase(d,prev,c)[0])) }
}

#[verifier::opaque]
spec fn lz_bits(s: Seq<u8>, pos: int, n: nat) -> int
    decreases n,
{
    if n == 0 { 0 } else {
        2 * lz_bits(s,pos,(n-1) as nat)
            + (s[(pos+n-1)/8] as int / power2(7-(pos+n-1)%8)) % 2
    }
}

#[verifier::opaque]
spec fn lz_lsb_bits(s: Seq<u8>, pos: int, n: nat) -> int
    decreases n,
{
    if n == 0 { 0 } else {
        lz_lsb_bits(s,pos,(n-1) as nat)
            + pow2_code(n-1) * ((s[(pos+n-1)/8] as int / power2((pos+n-1)%8)) % 2)
    }
}
spec fn lz_wire_bits(s: Seq<u8>, pos: int, n: nat, legacy: bool) -> int {
    if legacy { lz_lsb_bits(s,pos,n) } else { lz_bits(s,pos,n) }
}
#[verifier::opaque]
spec fn lz_legacy(s: Seq<u8>, start: int, end: int) -> bool {
    0<=start && start+2<=end<=s.len() && s[start]==0 && s[start+1]%2==1
}
fn lz_is_legacy(s:&[u8],start:usize,count:usize) -> (r:bool)
    requires range_ok(s.len() as int,start as int,count as int),
    ensures r == lz_legacy(s@,start as int,start as int+count),
{
    reveal(lz_legacy);
    count>=2 && s[start]==0 && s[start+1]%2==1
}
fn lz_weight(i:usize) -> (r:usize)
    requires i<12,
    ensures r as int==pow2_code(i as int), 1<=r<=2048,
{
    if i==0 {1} else if i==1 {2} else if i==2 {4} else if i==3 {8}
    else if i==4 {16} else if i==5 {32} else if i==6 {64} else if i==7 {128}
    else if i==8 {256} else if i==9 {512} else if i==10 {1024} else {2048}
}

#[derive(Clone, Copy)]
struct LzBits { at: usize, bit: usize, end: usize, legacy: bool }
impl LzBits {
    spec fn pos(self) -> int { self.at as int*8 + self.bit }
    spec fn valid(self, s: Seq<u8>) -> bool {
        self.at <= self.end <= s.len() && self.bit < 8
        && (self.at == self.end ==> self.bit == 0)
    }
    // Only a sub-byte remainder can stand in for an omitted/truncated EOI.
    fn at_tail(&self) -> (r: bool)
        requires self.at <= self.end, self.bit < 8,
            self.at == self.end ==> self.bit == 0,
        ensures r == (0 <= self.end as int*8-self.pos() < 8),
    { self.at == self.end || (self.bit > 0 && self.end-self.at == 1) }
    #[inline(always)]
    fn read(&mut self, s: &[u8], n: usize) -> (r: Result<usize,Error>)
        requires old(self).valid(s@), 1 <= n <= 12,
        ensures final(self).valid(s@), final(self).end == old(self).end,
            final(self).legacy == old(self).legacy,
            final(self).pos() >= old(self).pos(),
            r.is_ok() == (old(self).pos()+n <= old(self).end as int*8),
            r.is_ok() ==> final(self).pos() == old(self).pos()+n,
            r.is_ok() ==> r.unwrap() as int == lz_wire_bits(s@,old(self).pos(),n as nat,old(self).legacy),
            r.is_ok() ==> r.unwrap() < 4096,
    {
        if self.end-self.at>=3 {
            let v=bit_window::read(s,self.at,self.bit,n,!self.legacy);
            let next=self.bit+n;
            proof {
                quotient_remainder(next as int,8);
                assert(deflate_bits::pow(n as int)<=4096);
            }
            self.at=self.at+next/8;self.bit=next%8;
            return Ok(v);
        }
        let mut v=0usize;
        proof { reveal_with_fuel(lz_bits,1); reveal_with_fuel(lz_lsb_bits,1); }
        let mut i=0usize;
        while i<n
            invariant self.valid(s@), self.end == old(self).end, 1<=n<=12, i<=n,
                self.legacy == old(self).legacy,
                self.pos() == old(self).pos()+i,
                v as int == lz_wire_bits(s@,old(self).pos(),i as nat,self.legacy),
                v < pow2_code(i as int),
            decreases n-i,
        {
            if self.at == self.end { return Err(Error::Truncated); }
            let b=(s[self.at]/bit_divisor(if self.legacy {self.bit} else {7-self.bit}))%2;
            proof { pow2_code_step(i as int); }
            if self.legacy {
                let weight=lz_weight(i);
                if b==1 {v=v+weight;}
            } else {v=v*2+b as usize;}
            if self.bit == 7 { self.at=self.at+1; self.bit=0; }
            else { self.bit=self.bit+1; }
            i=i+1;
            proof { reveal_with_fuel(lz_bits,1); reveal_with_fuel(lz_lsb_bits,1); }
        }
        proof { pow2_code_bound(n as int); }
        Ok(v)
    }
}
spec fn pow2_code(n: int) -> int {
    if n<=0 {1} else if n==1 {2} else if n==2 {4} else if n==3 {8}
    else if n==4 {16} else if n==5 {32} else if n==6 {64} else if n==7 {128}
    else if n==8 {256} else if n==9 {512} else if n==10 {1024} else if n==11 {2048} else {4096}
}
proof fn pow2_code_step(n: int)
    requires 0<=n<12,
    ensures pow2_code(n+1)==2*pow2_code(n), pow2_code(n)<=2048,
{ }
proof fn pow2_code_bound(n: int)
    requires 0<=n<=12,
    ensures pow2_code(n)<=4096,
{ }

#[derive(Clone,Copy)]
struct LzEntry { parent: u16, tail: u8, first: u8, len: u16 }
struct LzTable {
    entries: [LzEntry;4096], next: usize, prev: usize,
    _dict: Ghost<Seq<Seq<u8>>>,
}
spec fn lz_entry_valid(d:Seq<Seq<u8>>,e:LzEntry,i:int) -> bool {
    &&& lz_defined(d,e.parent as int)
    &&& e.parent<i
    &&& d[i] == d[e.parent as int].push(e.tail)
    &&& e.len == d[i].len()
    &&& e.first == d[i][0]
    &&& 1<=e.len<=i+1
}
impl LzTable {
    spec fn valid(&self) -> bool {
        &&& 258 <= self.next <= 4096
        &&& self._dict@.len() == self.next
        &&& (self.prev == 256 || lz_defined(self._dict@,self.prev as int))
        &&& (forall|i: int| 0<=i<256 ==> #[trigger] self._dict@[i] == seq![i as u8])
        &&& (forall|i: int| 258<=i<self.next ==> #[trigger] lz_entry_valid(self._dict@,self.entries@[i],i))
    }
    fn new() -> (r: Self)
        ensures r.valid(), r._dict@ == lz_initial(), r.prev == 256,
    {
        let ghost dict=lz_initial();
        Self { entries:[LzEntry{parent:0,tail:0,first:0,len:0};4096], next:258, prev:256,
            _dict:Ghost(dict) }
    }
    fn clear(&mut self)
        requires old(self).valid(),
        ensures final(self).valid(), final(self)._dict@ == lz_initial(), final(self).prev == 256,
    { self.next=258; self.prev=256; self._dict=Ghost(lz_initial()); }
    #[inline(always)]
    fn width(&self,legacy:bool) -> (r: usize)
        requires self.valid(),
        ensures r as int == lz_width(self._dict@.len() as int,legacy), 9<=r<=12,
    {
        if legacy {if self.next<512 {9} else if self.next<1024 {10} else if self.next<2048 {11} else {12}}
        else {if self.next<511 {9} else if self.next<1023 {10} else if self.next<2047 {11} else {12}}
    }
    proof fn entry(&self,c:int)
        requires self.valid(), lz_defined(self._dict@,c),
        ensures 1<=self._dict@[c].len()<=c+1,
            c>=258 ==> self.entries@[c].len == self._dict@[c].len()
                && self.entries@[c].first == self._dict@[c][0]
                && self.entries@[c].parent < c
                && lz_defined(self._dict@,self.entries@[c].parent as int)
                && self._dict@[c] == self._dict@[self.entries@[c].parent as int].push(self.entries@[c].tail),
    { if c>=258 { assert(lz_entry_valid(self._dict@,self.entries@[c],c)); } }
    fn info(&self,c:usize) -> (r:(usize,u8))
        requires self.valid(), lz_defined(self._dict@,c as int),
        ensures r.0 == self._dict@[c as int].len(), 1<=r.0<=c+1,
            r.1 == self._dict@[c as int][0],
    {
        proof { self.entry(c as int); }
        if c<256 { (1,c as u8) } else { (self.entries[c].len as usize,self.entries[c].first) }
    }
    #[inline(always)]
    fn data(&mut self,c:usize,legacy:bool) -> (r:Result<usize,Error>)
        requires old(self).valid(), c<4096, c!=256, c!=257,
        ensures final(self).valid(),
            r.is_ok() == lz_data_ok(old(self)._dict@,old(self).prev as int,c as int,legacy),
            r.is_ok() ==> final(self)._dict@ == lz_after(old(self)._dict@,old(self).prev as int,c as int),
            r.is_ok() ==> final(self).prev == c,
            r.is_ok() ==> lz_defined(final(self)._dict@,c as int),
            r.is_ok() ==> final(self)._dict@[c as int] == lz_phrase(old(self)._dict@,old(self).prev as int,c as int),
            r.is_ok() ==> r.unwrap() == final(self)._dict@[c as int].len(),
            r.is_ok() ==> 1<=r.unwrap()<=4096,
    {
        if self.prev!=256 && self.next==4096 && !legacy { return Err(Error::Invalid); }
        if c>self.next || (c==self.next && self.prev==256) { return Err(Error::Invalid); }
        let first=if c==self.next { self.info(self.prev).1 } else { self.info(c).1 };
        let ghost prior=*self;
        let ghost d=self._dict@;
        if self.prev!=256 && self.next<4096 {
            let (len,pfirst)=self.info(self.prev);
            let new_entry=LzEntry{parent:self.prev as u16,tail:first,first:pfirst,len:(len+1) as u16};
            self.entries[self.next]=new_entry;
            self._dict=Ghost(self._dict@.push(self._dict@[self.prev as int].push(first)));
            self.next=self.next+1;
        }
        self.prev=c;
        assert forall|i:int| 0<=i<256 implies #[trigger] self._dict@[i] == seq![i as u8] by {
            assert(d[i] == seq![i as u8]);
        }
        assert forall|i:int| 258<=i<self.next implies
            #[trigger] lz_entry_valid(self._dict@,self.entries@[i],i) by {
            if i<prior.next { prior.entry(i); }
            else { prior.entry(prior.prev as int); }
        }
        assert(self.valid());
        let (len,_)=self.info(c);
        Ok(len)
    }
    #[inline(always)]
    fn expand(&self,c:usize,buf:&mut [u8;4096]) -> (len:usize)
        requires self.valid(), lz_defined(self._dict@,c as int),
        ensures len == self._dict@[c as int].len(), 1<=len<=4096,
            forall|k:int| 0<=k<len ==> #[trigger] final(buf)@[k] == self._dict@[c as int][k],
    {
        let len=self.info(c).0;
        let mut code=c;
        let mut left=len;
        while code>=258
            invariant self.valid(), lz_defined(self._dict@,c as int),
                lz_defined(self._dict@,code as int),
                len == self._dict@[c as int].len(), 1<=len<=4096,
                left == self._dict@[code as int].len(), 1<=left<=len,
                forall|k:int| 0<=k<left ==> #[trigger] self._dict@[code as int][k] == self._dict@[c as int][k],
                forall|k:int| left<=k<len ==> #[trigger] buf@[k] == self._dict@[c as int][k],
            decreases code,
        {
            proof { self.entry(code as int); }
            let e=self.entries[code];
            proof { self.entry(e.parent as int); }
            let ghost before=code;
            let ghost old_buf=buf@;
            left=left-1;
            buf[left]=e.tail;
            code=e.parent as usize;
            proof {
                assert(self._dict@[before as int][left as int] == e.tail);
                assert(self._dict@[before as int][left as int] == self._dict@[c as int][left as int]);
                assert forall|k:int| left<=k<len implies #[trigger] buf@[k] == self._dict@[c as int][k] by {
                    if k==left {} else { assert(old_buf[k] == self._dict@[c as int][k]); }
                }
                assert forall|k:int| 0<=k<left implies #[trigger] self._dict@[code as int][k] == self._dict@[c as int][k] by {
                    assert(self._dict@[before as int][k] == self._dict@[c as int][k]);
                }
            }
        }
        proof { self.entry(code as int); }
        assert(left==1);
        assert(self._dict@[code as int][0] == self._dict@[c as int][0]);
        let ghost old_buf=buf@;
        buf[0]=code as u8;
        assert forall|k:int| 0<=k<len implies #[trigger] buf@[k] == self._dict@[c as int][k] by {
            if k!=0 { assert(old_buf[k] == self._dict@[c as int][k]); }
        }
        len
    }
}

// Pure stream semantics use byte strings, independently of the prefix-table
// representation. Each recursive step consumes one complete encoded code.
#[verifier::opaque]
spec fn lz_valid(s:Seq<u8>,pos:int,end:int,d:Seq<Seq<u8>>,prev:int,left:int,legacy:bool) -> bool
    decreases end*8-pos,
{
    let width=lz_width(d.len() as int,legacy);
    &&& 0<=pos<=end*8 && 0<=end<=s.len() && left>=0
    &&& if left==0 && prev!=256 && end*8-pos<8 {
        true
    } else if pos+width>end*8 { false } else {
        let c=lz_wire_bits(s,pos,width as nat,legacy);
        if c==256 {
        lz_valid(s,pos+width,end,lz_initial(),256,left,legacy)
    } else if c==257 {
        left==0 && end*8-(pos+width)<8
    } else {
        lz_data_ok(d,prev,c,legacy) && lz_phrase(d,prev,c).len()<=left
        && lz_valid(s,pos+width,end,lz_after(d,prev,c),c,left-lz_phrase(d,prev,c).len(),legacy)
    } }
}
#[verifier::opaque]
spec fn lz_byte(s:Seq<u8>,pos:int,end:int,d:Seq<Seq<u8>>,prev:int,k:int,legacy:bool) -> u8
    decreases end*8-pos,
{
    let width=lz_width(d.len() as int,legacy);
    let c=lz_wire_bits(s,pos,width as nat,legacy);
    if pos<0 || end>s.len() || pos+width>end*8 || k<0 || c==257 {0u8}
    else if c==256 { lz_byte(s,pos+width,end,lz_initial(),256,k,legacy) }
    else if !lz_data_ok(d,prev,c,legacy) {0u8}
    else if k<lz_phrase(d,prev,c).len() { lz_phrase(d,prev,c)[k] }
    else { lz_byte(s,pos+width,end,lz_after(d,prev,c),c,k-lz_phrase(d,prev,c).len(),legacy) }
}
#[verifier::opaque]
spec fn lzw_valid(s:Seq<u8>,start:int,end:int,needed:int) -> bool {
    0<=start<=end<=s.len() && start*8+9<=end*8
    && lz_wire_bits(s,start*8,9,lz_legacy(s,start,end))==256
    && lz_valid(s,start*8+9,end,lz_initial(),256,needed,lz_legacy(s,start,end))
}
#[verifier::opaque]
spec fn lzw_byte(s:Seq<u8>,start:int,end:int,k:int) -> u8 {
    lz_byte(s,start*8+9,end,lz_initial(),256,k,lz_legacy(s,start,end))
}

// Clear, literal 123, then six residual bits: no complete EOI is present.
// The same bytes may not supply two pixels or acquire a whole trailing byte.
proof fn lzw_tail_has_witness()
    ensures lzw_valid(seq![128u8,30u8,224u8],0,3,1),
        lzw_byte(seq![128u8,30u8,224u8],0,3,0)==123u8,
        !lzw_valid(seq![128u8,30u8,224u8],0,3,2),
        !lzw_valid(seq![128u8,30u8,224u8,0u8],0,4,1),
{
    assert(lzw_valid(seq![128u8,30u8,224u8],0,3,1)) by(compute);
    assert(lzw_byte(seq![128u8,30u8,224u8],0,3,0)==123u8) by(compute);
    assert(!lzw_valid(seq![128u8,30u8,224u8],0,3,2)) by(compute);
    assert(!lzw_valid(seq![128u8,30u8,224u8,0u8],0,4,1)) by(compute);
}

fn lz_validate(s:&[u8],start:usize,count:usize,needed:usize,budget:&mut Budget) -> (r:Result<(),Error>)
    requires range_ok(s.len() as int,start as int,count as int),
    ensures final(budget).remaining<=old(budget).remaining,
        r.is_ok() == (lzw_valid(s@,start as int,start as int+count,needed as int)
            && old(budget).remaining>=count),
        r.is_ok() ==> final(budget).remaining as int+count==old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=count,
{
    reveal(lzw_valid);
    spend(budget,count)?;
    let legacy=lz_is_legacy(s,start,count);
    let mut bits=LzBits{at:start,bit:0,end:start+count,legacy};
    if bits.read(s,9)?!=256 { return Err(Error::Invalid); }
    let mut table=LzTable::new();
    let mut left=needed;
    loop
        invariant bits.valid(s@), table.valid(), bits.end==start+count,
            bits.legacy==lz_legacy(s@,start as int,start as int+count),
            budget.remaining as int+count==old(budget).remaining,
            lzw_valid(s@,start as int,start as int+count,needed as int)
                == lz_valid(s@,bits.pos(),bits.end as int,table._dict@,table.prev as int,left as int,bits.legacy),
        decreases bits.end as int*8-bits.pos(),
    {
        reveal_with_fuel(lz_valid,1);
        if left==0 && table.prev!=256 && bits.at_tail() { return Ok(()); }
        let c=bits.read(s,table.width(bits.legacy))?;
        if c==256 { table.clear(); }
        else if c==257 {
            if left!=0 || (bits.at<bits.end && bits.end-bits.at>1)
                || (bits.at<bits.end && bits.bit==0) { return Err(Error::Invalid); }
            return Ok(());
        } else {
            let len=table.data(c,bits.legacy)?;
            if len>left { return Err(Error::Invalid); }
            left=left-len;
        }
    }
}

struct LzCursor {
    table:LzTable, bits:LzBits, buf:[u8;4096], used:usize, len:usize,
    emitted:usize,
    ghost _start:usize, ghost _needed:usize,
}
impl LzCursor {
    spec fn valid(&self,s:Seq<u8>) -> bool {
        &&& self.table.valid() && self.bits.valid(s)
        &&& self.bits.legacy==lz_legacy(s,self._start as int,self.bits.end as int)
        &&& self._start<=self.bits.end && self._start as int*8+9<=self.bits.pos()
        &&& self.emitted<=self._needed && self.used<=self.len<=4096
        &&& self.len-self.used<=self._needed-self.emitted
        &&& lz_valid(s,self.bits.pos(),self.bits.end as int,self.table._dict@,self.table.prev as int,
            self._needed as int-self.emitted-(self.len-self.used),self.bits.legacy)
        &&& (forall|k:int| 0<=k<self.len-self.used ==>
            #[trigger] lzw_byte(s,self._start as int,self.bits.end as int,self.emitted as int+k)
                == self.buf@[self.used as int+k])
        &&& (forall|k:int| self.len-self.used<=k<self._needed-self.emitted ==>
            #[trigger] lzw_byte(s,self._start as int,self.bits.end as int,self.emitted as int+k)
                == lz_byte(s,self.bits.pos(),self.bits.end as int,self.table._dict@,self.table.prev as int,k-(self.len-self.used),self.bits.legacy))
    }
    fn new(s:&[u8],start:usize,count:usize,_needed:usize) -> (r:Self)
        requires range_ok(s.len() as int,start as int,count as int),
            lzw_valid(s@,start as int,start as int+count,_needed as int),
        ensures r.valid(s@), r._start==start, r._needed==_needed,
            r.bits.end==start+count, r.emitted==0,
    {
        reveal(lzw_valid);
        reveal(lzw_byte);
        let legacy=lz_is_legacy(s,start,count);
        let mut bits=LzBits{at:start,bit:0,end:start+count,legacy};
        let _header=bits.read(s,9);
        assert(_header.is_ok());
        Self{table:LzTable::new(),bits,buf:[0u8;4096],used:0,len:0,emitted:0,_start:start,_needed}
    }
    proof fn pending(&self,s:Seq<u8>,k:int)
        requires self.valid(s), 0<=k<self.len-self.used,
        ensures lzw_byte(s,self._start as int,self.bits.end as int,self.emitted as int+k)
            == self.buf@[self.used as int+k],
    {}
    proof fn tail(&self,s:Seq<u8>,k:int)
        requires self.valid(s), self.len-self.used<=k<self._needed-self.emitted,
        ensures lzw_byte(s,self._start as int,self.bits.end as int,self.emitted as int+k)
            == lz_byte(s,self.bits.pos(),self.bits.end as int,self.table._dict@,self.table.prev as int,k-(self.len-self.used),self.bits.legacy),
    {}
    fn next(&mut self,s:&[u8]) -> (v:u8)
        requires old(self).valid(s@), old(self).emitted<old(self)._needed,
        ensures final(self).valid(s@), final(self)._start==old(self)._start,
            final(self)._needed==old(self)._needed, final(self).bits.end==old(self).bits.end,
            final(self).emitted==old(self).emitted+1,
            final(self).bits.pos()>=old(self).bits.pos(),
            v==lzw_byte(s@,old(self)._start as int,old(self).bits.end as int,old(self).emitted as int),
    {
        while self.used==self.len
            invariant self.valid(s@), self.emitted<self._needed,
                self._start==old(self)._start, self._needed==old(self)._needed,
                self.bits.end==old(self).bits.end, self.emitted==old(self).emitted,
                self.bits.pos()>=old(self).bits.pos(),
            decreases self.bits.end as int*8-self.bits.pos(),
        {
            reveal_with_fuel(lz_valid,1);
            let ghost before=*self;
            let result=self.bits.read(s,self.table.width(self.bits.legacy));
            assert(result.is_ok());
            let c=match result {Ok(c)=>c,Err(_)=>{return 0;}};
            assert(c!=257);
            if c==256 {
                self.table.clear();
                proof {
                    assert forall|k:int| 0<=k<self._needed-self.emitted implies
                        #[trigger] lzw_byte(s@,self._start as int,self.bits.end as int,self.emitted as int+k)
                            == lz_byte(s@,self.bits.pos(),self.bits.end as int,self.table._dict@,self.table.prev as int,k,self.bits.legacy) by {
                        before.tail(s@,k);
                        reveal_with_fuel(lz_byte,1);
                    }
                }
            } else {
                let _result=self.table.data(c,self.bits.legacy);
                assert(_result.is_ok());
                self.len=self.table.expand(c,&mut self.buf);
                self.used=0;
                proof {
                    assert forall|k:int| 0<=k<self.len implies
                        #[trigger] lzw_byte(s@,self._start as int,self.bits.end as int,self.emitted as int+k)
                            == self.buf@[k] by {
                        before.tail(s@,k);
                        reveal_with_fuel(lz_byte,1);
                    }
                    assert forall|k:int| self.len<=k<self._needed-self.emitted implies
                        #[trigger] lzw_byte(s@,self._start as int,self.bits.end as int,self.emitted as int+k)
                            == lz_byte(s@,self.bits.pos(),self.bits.end as int,self.table._dict@,self.table.prev as int,k-self.len,self.bits.legacy) by {
                        before.tail(s@,k);
                        reveal_with_fuel(lz_byte,1);
                    }
                }
            }
        }
        let v=self.buf[self.used];
        let ghost before=*self;
        proof { before.pending(s@,0); }
        self.used=self.used+1;
        self.emitted=self.emitted+1;
        proof {
            assert forall|k:int| 0<=k<self.len-self.used implies
                #[trigger] lzw_byte(s@,self._start as int,self.bits.end as int,self.emitted as int+k)
                    == self.buf@[self.used as int+k] by { before.pending(s@,k+1); }
            assert forall|k:int| self.len-self.used<=k<self._needed-self.emitted implies
                #[trigger] lzw_byte(s@,self._start as int,self.bits.end as int,self.emitted as int+k)
                    == lz_byte(s@,self.bits.pos(),self.bits.end as int,self.table._dict@,self.table.prev as int,k-(self.len-self.used),self.bits.legacy) by {
                before.tail(s@,k+1);
            }
        }
        v
    }
}

// PackBits is specified directly as a sequence of literal, replicate, and
// no-op packets. Each nonempty packet must stay within its decoded scanline.
spec fn pb_len(s: Seq<u8>, at: int) -> int {
    let n = s[at] as int;
    if n < 128 { n + 1 } else if n == 128 { 0 } else { 257 - n }
}

spec fn pb_payload(s: Seq<u8>, at: int) -> int {
    if s[at] < 128 { pb_len(s, at) } else if s[at] == 128 { 0 } else { 1 }
}

spec fn pb_next(s: Seq<u8>, at: int) -> int { at + 1 + pb_payload(s, at) }

spec fn pb_packet_ok(s: Seq<u8>, at: int, end: int) -> bool {
    0 <= at < end <= s.len() && pb_next(s, at) <= end
}

#[verifier::opaque]
spec fn pb_valid(s: Seq<u8>, at: int, end: int, stride: int, left: int) -> bool
    decreases end - at,
{
    0 <= at <= end <= s.len() && stride > 0 && left >= 0
    && if at == end { left == 0 } else {
        pb_packet_ok(s, at, end)
        && pb_len(s, at) <= left
        && pb_len(s, at) <= (if left % stride == 0 { stride } else { left % stride })
        && pb_valid(s, pb_next(s, at), end, stride, left - pb_len(s, at))
    }
}

#[verifier::opaque]
spec fn pb_byte(s: Seq<u8>, at: int, end: int, k: int) -> u8
    decreases end - at,
{
    if k < 0 || !pb_packet_ok(s, at, end) { 0u8 }
    else if k < pb_len(s, at) {
        if s[at] < 128 { s[at + 1 + k] } else { s[at + 1] }
    } else { pb_byte(s, pb_next(s, at), end, k - pb_len(s, at)) }
}

#[derive(Clone, Copy)]
struct PbPacket { data: usize, next: usize, len: usize, repeat: bool }

fn pb_packet(s: &[u8], at: usize, end: usize) -> (r: Result<PbPacket, Error>)
    requires end <= s.len(),
    ensures r.is_ok() == pb_packet_ok(s@, at as int, end as int),
        r.is_ok() ==> r.unwrap().data == at + 1,
        r.is_ok() ==> r.unwrap().next == pb_next(s@, at as int),
        r.is_ok() ==> at < r.unwrap().next <= end,
        r.is_ok() ==> r.unwrap().len == pb_len(s@, at as int),
        r.is_ok() ==> r.unwrap().len <= 128,
        r.is_ok() ==> r.unwrap().repeat == (s@[at as int] > 128),
        r.is_ok() ==> (if r.unwrap().repeat { r.unwrap().data < end }
            else { r.unwrap().data + r.unwrap().len <= end }),
{
    if at >= end { return Err(Error::Truncated); }
    let n = s[at] as usize;
    let data = at + 1;
    let len = if n < 128 { n + 1 } else if n == 128 { 0 } else { 257 - n };
    let payload = if n < 128 { len } else if n == 128 { 0 } else { 1 };
    if payload > end - data { return Err(Error::Truncated); }
    Ok(PbPacket { data, next: data + payload, len, repeat: n > 128 })
}

fn pb_validate(s: &[u8], start: usize, count: usize, stride: usize,
    needed: usize, budget: &mut Budget) -> (r: Result<(), Error>)
    requires range_ok(s.len() as int, start as int, count as int), stride > 0,
    ensures final(budget).remaining <= old(budget).remaining,
        r.is_ok() == (pb_valid(s@, start as int, start as int + count, stride as int, needed as int)
            && old(budget).remaining >= count),
        r.is_ok() ==> final(budget).remaining as int + count == old(budget).remaining,
        old(budget).remaining as int - final(budget).remaining <= count,
{
    spend(budget, count)?;
    let end = start + count;
    let mut at = start;
    let mut left = needed;
    while at < end
        invariant start <= at <= end, end == start + count, end <= s.len(), stride > 0,
            budget.remaining as int + count == old(budget).remaining,
            pb_valid(s@, start as int, end as int, stride as int, needed as int)
                == pb_valid(s@, at as int, end as int, stride as int, left as int),
        decreases end - at,
    {
        reveal_with_fuel(pb_valid, 1);
        let p = pb_packet(s, at, end)?;
        if p.len > left { return Err(Error::Invalid); }
        let row_left = if left % stride == 0 { stride } else { left % stride };
        if p.len > row_left { return Err(Error::Invalid); }
        left = left - p.len;
        at = p.next;
    }
    reveal_with_fuel(pb_valid, 1);
    if left != 0 { return Err(Error::Truncated); }
    Ok(())
}

// A streaming cursor retains one PackBits packet, never the expanded segment.
// Its ghost invariant relates the unread suffix to the original byte model.
#[derive(Clone, Copy)]
struct PbCursor {
    ghost _start: usize,
    end: usize,
    ghost _stride: usize,
    ghost _needed: usize,
    at: usize, emitted: usize, data: usize, run: usize, repeat: bool,
}

impl PbCursor {
    spec fn valid(self, s: Seq<u8>) -> bool {
        &&& self._start <= self.at <= self.end <= s.len()
        &&& self._stride > 0
        &&& self.emitted <= self._needed
        &&& self.run <= self._needed - self.emitted
        &&& (if self.repeat { self.run > 0 ==> self.data < self.end }
            else { self.data as int + self.run <= self.end })
        &&& pb_valid(s, self.at as int, self.end as int, self._stride as int,
            self._needed as int - self.emitted - self.run)
        &&& (forall|k: int| 0 <= k < self.run ==>
            #[trigger] pb_byte(s, self._start as int, self.end as int, self.emitted as int + k)
                == s[self.data as int + if self.repeat { 0int } else { k }])
        &&& (forall|k: int| self.run <= k < self._needed - self.emitted ==>
            #[trigger] pb_byte(s, self._start as int, self.end as int, self.emitted as int + k)
                == pb_byte(s, self.at as int, self.end as int, k - self.run))
    }

    proof fn run_byte(self, s: Seq<u8>, k: int)
        requires self.valid(s), 0 <= k < self.run,
        ensures pb_byte(s, self._start as int, self.end as int, self.emitted as int + k)
            == s[self.data as int + if self.repeat { 0int } else { k }],
    {}

    proof fn tail_byte(self, s: Seq<u8>, k: int)
        requires self.valid(s), self.run <= k < self._needed - self.emitted,
        ensures pb_byte(s, self._start as int, self.end as int, self.emitted as int + k)
            == pb_byte(s, self.at as int, self.end as int, k - self.run),
    {}

    fn new(_s: &[u8], start: usize, count: usize, _stride: usize, _needed: usize) -> (r: Self)
        requires range_ok(_s.len() as int, start as int, count as int), _stride > 0,
            pb_valid(_s@, start as int, start as int + count, _stride as int, _needed as int),
        ensures r.valid(_s@), r.emitted == 0, r._start == start,
            r.end == start + count, r._needed == _needed, r._stride == _stride,
    {
        Self { _start: start, end: start + count, _stride: _stride, _needed: _needed, at: start,
            emitted: 0, data: start, run: 0, repeat: false }
    }

    fn next(&mut self, s: &[u8]) -> (v: u8)
        requires old(self).valid(s@), old(self).emitted < old(self)._needed,
        ensures final(self).valid(s@), final(self)._start == old(self)._start,
            final(self).end == old(self).end, final(self)._stride == old(self)._stride,
            final(self)._needed == old(self)._needed,
            final(self).emitted == old(self).emitted + 1,
            final(self).at >= old(self).at,
            v == pb_byte(s@, old(self)._start as int, old(self).end as int, old(self).emitted as int),
    {
        while self.run == 0
            invariant self.valid(s@), self.emitted < self._needed,
                self._start == old(self)._start, self.end == old(self).end,
                self._stride == old(self)._stride, self._needed == old(self)._needed,
                self.emitted == old(self).emitted, self.at >= old(self).at,
            decreases self.end - self.at,
        {
            reveal_with_fuel(pb_valid, 1);
            let ghost before = *self;
            let packet = pb_packet(s, self.at, self.end);
            assert(packet.is_ok());
            let p = match packet { Ok(p) => p, Err(_) => { return 0; } };
            proof {
                assert forall|k: int| 0 <= k < p.len implies
                    #[trigger] pb_byte(s@, self._start as int, self.end as int, self.emitted as int + k)
                        == s@[p.data as int + if p.repeat { 0int } else { k }] by {
                    reveal_with_fuel(pb_byte, 1);
                }
                assert forall|k: int| p.len <= k < self._needed - self.emitted implies
                    #[trigger] pb_byte(s@, self._start as int, self.end as int, self.emitted as int + k)
                        == pb_byte(s@, p.next as int, self.end as int, k - p.len) by {
                    reveal_with_fuel(pb_byte, 1);
                }
            }
            self.data = p.data; self.run = p.len; self.repeat = p.repeat; self.at = p.next;
        }
        let v = s[self.data];
        let ghost before = *self;
        proof { before.run_byte(s@, 0); }
        if !self.repeat { self.data = self.data + 1; }
        self.run = self.run - 1;
        self.emitted = self.emitted + 1;
        proof {
            assert forall|k: int| 0 <= k < self.run implies
                #[trigger] pb_byte(s@, self._start as int, self.end as int, self.emitted as int + k)
                    == s@[self.data as int + if self.repeat { 0int } else { k }] by {
                assert(0 <= k + 1 < before.run);
                before.run_byte(s@, k + 1);
            }
            assert forall|k: int| self.run <= k < self._needed - self.emitted implies
                #[trigger] pb_byte(s@, self._start as int, self.end as int, self.emitted as int + k)
                    == pb_byte(s@, self.at as int, self.end as int, k - self.run) by {
                assert(before.run <= k + 1 < before._needed - before.emitted);
                before.tail_byte(s@, k + 1);
            }
        }
        v
    }
}

#[derive(Debug, Clone, Copy)]
struct Geometry {
    width: usize, height: usize, channels: usize, alpha: usize,
    block_width: usize, block_height: usize, across: usize,
    offsets: Tag, counts: Tag, le: bool, white: bool, bilevel: bool, lsb: bool, packbits: bool, lzw: bool, tiled: bool,
}

impl Geometry {
    spec fn positive(self) -> bool {
        self.width > 0 && self.height > 0 && self.channels > 0
        && self.block_width > 0 && self.block_height > 0 && self.across > 0
        && (self.bilevel ==> self.channels == 1)
        && !(self.packbits && self.lzw)
        && self.alpha<=2
        && (self.alpha!=0 ==> !self.bilevel && (self.channels==2 || self.channels==4))
    }
    spec fn segment(self, k: int) -> int {
        ((k / (self.channels as int)) / (self.width as int) / (self.block_height as int)) * self.across
        + ((k / (self.channels as int)) % (self.width as int) / (self.block_width as int))
    }
    spec fn local(self, k: int) -> int {
        if self.bilevel {
            (((k / (self.channels as int)) / (self.width as int)) % (self.block_height as int)) * self.packed_row_bytes()
                + (((k / (self.channels as int)) % (self.width as int)) % (self.block_width as int)) / 8
        } else {
            (((k / (self.channels as int)) / (self.width as int) % (self.block_height as int)) * self.block_width
            + ((k / (self.channels as int)) % (self.width as int) % (self.block_width as int))) * self.channels
            + k % (self.channels as int)
        }
    }
    spec fn packed_row_bytes(self) -> int { (self.block_width as int - 1) / 8 + 1 }
    spec fn row_bytes(self) -> int {
        if self.bilevel { self.packed_row_bytes() } else { self.block_width as int * self.channels }
    }
    spec fn bit_position(self, k: int) -> int {
        let col = ((k / (self.channels as int)) % (self.width as int)) % (self.block_width as int);
        if self.lsb { col % 8 } else { 7 - col % 8 }
    }
}

spec fn tag_value(s: Seq<u8>, t: Tag, i: int, le: bool) -> int {
    if t.kind == 3 { word16(s, t.data as int + i * 2, le) }
    else { word32(s, t.data as int + i * 4, le) }
}

spec fn sample_address(s: Seq<u8>, g: Geometry, k: int) -> int {
    tag_value(s, g.offsets, g.segment(k), g.le) + g.local(k)
}

spec fn power2(bit: int) -> int {
    if bit == 0 { 1 } else if bit == 1 { 2 } else if bit == 2 { 4 }
    else if bit == 3 { 8 } else if bit == 4 { 16 } else if bit == 5 { 32 }
    else if bit == 6 { 64 } else { 128 }
}

fn bit_divisor(bit: usize) -> (r: u8)
    requires bit < 8,
    ensures r as int == power2(bit as int), r > 0,
{
    if bit == 0 { 1 } else if bit == 1 { 2 } else if bit == 2 { 4 }
    else if bit == 3 { 8 } else if bit == 4 { 16 } else if bit == 5 { 32 }
    else if bit == 6 { 64 } else { 128 }
}

spec fn storage_pixel(s: Seq<u8>, g: Geometry, k: int) -> u8 {
    let start = tag_value(s, g.offsets, g.segment(k), g.le);
    let end = start + tag_value(s, g.counts, g.segment(k), g.le);
    let byte = if g.packbits { pb_byte(s, start, end, g.local(k)) } else if g.lzw { lzw_byte(s,start,end,g.local(k)) } else { s[sample_address(s, g, k)] };
    let v = if g.bilevel {
        if (byte as int / power2(g.bit_position(k))) % 2 == 0 { 0u8 } else { 255u8 }
    } else { byte };
    if g.white && g.alpha==0 { (255 - v) as u8 } else { v }
}

spec fn address_arithmetic_fits(g: Geometry, k: int) -> bool {
    let p = k / (g.channels as int);
    let x = p % (g.width as int);
    let y = p / (g.width as int);
    let row = (y % (g.block_height as int)) * g.block_width;
    let cell = row + x % (g.block_width as int);
    let base = cell * g.channels;
    &&& 0 <= (y / (g.block_height as int)) * g.across <= usize::MAX
    &&& 0 <= g.segment(k) <= usize::MAX
    &&& (if g.bilevel {
        0 <= (y % (g.block_height as int)) * g.packed_row_bytes() <= usize::MAX
    } else {
        0 <= row <= usize::MAX && 0 <= cell <= usize::MAX && 0 <= base <= usize::MAX
    })
    &&& 0 <= g.local(k) <= usize::MAX
}

spec fn sample_valid(s: Seq<u8>, g: Geometry, k: int) -> bool {
    g.positive() && 0 <= k && address_arithmetic_fits(g, k)
    && value_valid(s, g.offsets, g.segment(k))
    && value_valid(s, g.counts, g.segment(k))
    && if g.packbits || g.lzw {
        let start = tag_value(s, g.offsets, g.segment(k), g.le);
        let count = tag_value(s, g.counts, g.segment(k), g.le);
        let needed = storage_bytes(g, g.tiled, g.segment(k));
        0 <= g.local(k) < needed
        && (if g.packbits { pb_valid(s, start, start + count, g.row_bytes(), needed) }
            else { lzw_valid(s,start,start+count,needed) })
    } else {
        0 <= g.local(k) < tag_value(s, g.counts, g.segment(k), g.le)
        && 0 <= sample_address(s, g, k) <= usize::MAX
        && range_ok(s.len() as int, sample_address(s, g, k), 1)
    }
}

fn sample(s: &[u8], g: Geometry, k: usize) -> (r: Result<u8, Error>)
    requires g.positive(), !(g.packbits || g.lzw),
    ensures r.is_ok() == sample_valid(s@, g, k as int),
        r.is_ok() ==> r.unwrap() == storage_pixel(s@, g, k as int),
        r.is_ok() ==> range_ok(s.len() as int, sample_address(s@, g, k as int), 1),
{
    let p = k / g.channels;
    let x = p % g.width;
    let y = p / g.width;
    let block = add(mul(y / g.block_height, g.across)?, x / g.block_width)?;
    let local = if g.bilevel {
        let row_bytes = (g.block_width - 1) / 8 + 1;
        add(mul(y % g.block_height, row_bytes)?, (x % g.block_width) / 8)?
    } else {
        add(mul(add(mul(y % g.block_height, g.block_width)?,
            x % g.block_width)?, g.channels)?, k % g.channels)?
    };
    let off = value(s, g.offsets, block, g.le)?;
    let count = value(s, g.counts, block, g.le)?;
    if local >= count { return Err(Error::Truncated); }
    let src = add(off, local)?;
    if src >= s.len() { return Err(Error::Truncated); }
    let v = if g.bilevel {
        let position = (x % g.block_width) % 8;
        let shift = if g.lsb { position } else { 7 - position };
        if (s[src] / bit_divisor(shift)) % 2 == 0 { 0u8 } else { 255u8 }
    } else { s[src] };
    if g.white && g.alpha==0 { Ok(255 - v) } else { Ok(v) }
}

// These are declarative predicates over the original bytes, never calls to
// the executable parser. Opaque boundaries keep each proof's SMT context small.
spec fn profile_geometry(s: Seq<u8>, d: Directory, le: bool) -> Geometry {
    let tiled = lookup(s, d, 324, le).is_some();
    let w = scalar_model(s, d, 256, 0, le) as usize;
    let bw = if tiled { scalar_model(s, d, 322, 0, le) as usize } else { w };
    Geometry {
        width: w, height: scalar_model(s, d, 257, 0, le) as usize,
        channels: scalar_model(s, d, 277, 1, le) as usize,
        alpha: scalar_model(s,d,338,0,le) as usize,
        block_width: bw,
        block_height: (if tiled { scalar_model(s, d, 323, 0, le) }
            else { scalar_model(s, d, 278, 4294967295, le) }) as usize,
        across: (((w as int - 1) / (bw as int)) + 1) as usize,
        offsets: lookup(s, d, if tiled { 324 } else { 273 }, le).unwrap(),
        counts: lookup(s, d, if tiled { 325 } else { 279 }, le).unwrap(),
        le, white: scalar_model(s, d, 262, 999, le) == 0,
        bilevel: bits_model(s, d, le) == 1,
        lsb: scalar_model(s, d, 266, 1, le) == 2,
        packbits: scalar_model(s, d, 259, 1, le) == 32773,
        lzw: scalar_model(s, d, 259, 1, le) == 5, tiled,
    }
}

spec fn array_of(s: Seq<u8>, t: Tag, n: int, v: int, le: bool) -> bool {
    t.kind == 3 && t.count == n && forall|i: int| 0 <= i < n ==>
        value_valid(s, t, i) && #[trigger] tag_value(s, t, i, le) == v
}

proof fn array_entry(s: Seq<u8>, t: Tag, n: int, v: int, le: bool, i: int)
    requires array_of(s, t, n, v, le), 0 <= i < n,
    ensures value_valid(s, t, i), tag_value(s, t, i, le) == v,
{
}

spec fn bits_model(s: Seq<u8>, d: Directory, le: bool) -> int {
    match lookup(s, d, 258, le) { None => 1, Some(t) => tag_value(s, t, 0, le) }
}

spec fn bits_supported(s: Seq<u8>, d: Directory, ch: int, le: bool) -> bool {
    let depth = bits_model(s, d, le);
    ((depth == 1 && ch == 1) || depth == 8)
    && match lookup(s, d, 258, le) {
        None => ch == 1,
        Some(t) => array_of(s, t, ch, depth, le),
    }
}

#[verifier::opaque]
spec fn profile_fields(s: Seq<u8>, d: Directory, le: bool, limits: Limits) -> bool {
    let w = scalar_model(s, d, 256, 0, le);
    let h = scalar_model(s, d, 257, 0, le);
    let ch = scalar_model(s, d, 277, 1, le);
    let photo = scalar_model(s, d, 262, 999, le);
    &&& scalar_valid(s, d, 256, le) && scalar_valid(s, d, 257, le)
    &&& scalar_valid(s, d, 277, le) && scalar_valid(s, d, 262, le)
    &&& 0 < w <= limits.max_dimension && 0 < h <= limits.max_dimension
    &&& (1 <= ch <= 4)
    &&& (if scalar_model(s,d,338,0,le)==0 {ch==1 || ch==3} else {ch==2 || ch==4})
    &&& ((ch <= 2 && (photo == 0 || photo == 1)) || (ch >= 3 && photo == 2))
    &&& 0 <= w * h <= usize::MAX && 0 <= w * h * ch <= limits.max_output
    &&& scalar_valid(s, d, 259, le)
    &&& (scalar_model(s, d, 259, 1, le) == 1 || scalar_model(s, d, 259, 1, le) == 32773 || scalar_model(s, d, 259, 1, le) == 5)
    &&& (scalar_model(s, d, 259, 1, le) != 1 ==> scalar_model(s, d, 266, 1, le) == 1)
    &&& scalar_valid(s, d, 274, le) && scalar_model(s, d, 274, 1, le) == 1
    &&& scalar_valid(s, d, 284, le) && scalar_model(s, d, 284, 1, le) == 1
    &&& scalar_valid(s, d, 266, le)
    &&& (scalar_model(s, d, 266, 1, le) == 1
        || (bits_model(s, d, le) == 1 && scalar_model(s, d, 266, 1, le) == 2))
    &&& scalar_valid(s, d, 317, le) && scalar_model(s, d, 317, 1, le) == 1
    &&& alpha_field_valid(s,d,le)
    &&& bits_supported(s, d, ch, le)
    &&& (match lookup(s, d, 339, le) {
        None => true, Some(t) => array_of(s, t, ch, 1, le),
    })
}

spec fn segment_count(g: Geometry) -> int {
    g.across as int * (((g.height as int - 1) / (g.block_height as int)) + 1)
}

spec fn stored_rows(g: Geometry, tiled: bool, j: int) -> int {
    if tiled { g.block_height as int }
    else { if g.height as int - j * g.block_height < g.block_height {
        g.height as int - j * g.block_height
    } else { g.block_height as int } }
}

spec fn storage_bytes(g: Geometry, tiled: bool, j: int) -> int {
    if g.bilevel { g.packed_row_bytes() * stored_rows(g,tiled,j) }
    else { g.block_width as int * stored_rows(g,tiled,j) * g.channels }
}

#[verifier::opaque]
spec fn segment_valid(s: Seq<u8>, g: Geometry, tiled: bool, j: int) -> bool {
    let start = tag_value(s, g.offsets, j, g.le);
    let count = tag_value(s, g.counts, j, g.le);
    let rows = stored_rows(g, tiled, j);
    &&& value_valid(s, g.offsets, j) && value_valid(s, g.counts, j)
    &&& range_ok(s.len() as int, start, count)
    &&& (!tiled ==> 0 <= j * g.block_height < g.height)
    &&& (if g.bilevel {
        0 <= g.packed_row_bytes() * rows <= usize::MAX

    } else {
        0 <= g.block_width as int * rows <= usize::MAX
        && 0 <= g.block_width as int * rows * g.channels <= usize::MAX
    })
    &&& 0 < g.row_bytes() <= usize::MAX
    &&& if g.packbits {
        pb_valid(s, start, start + count, g.row_bytes(), storage_bytes(g, tiled, j))
    } else if g.lzw { lzw_valid(s,start,start+count,storage_bytes(g,tiled,j)) }
    else { count >= storage_bytes(g, tiled, j) }
}

#[verifier::opaque]
spec fn codec_work_prefix(s: Seq<u8>, g: Geometry, n: nat) -> int
    decreases n,
{
    if n == 0 { 0 } else {
        codec_work_prefix(s,g,(n-1) as nat)
            + tag_value(s,g.counts,n as int-1,g.le)
            + storage_bytes(g,g.tiled,n as int-1)
    }
}

proof fn codec_work_zero(s: Seq<u8>,g: Geometry)
    ensures codec_work_prefix(s,g,0) == 0,
{ reveal_with_fuel(codec_work_prefix,1); }

proof fn codec_work_step(s: Seq<u8>,g: Geometry,n: nat)
    ensures codec_work_prefix(s,g,n+1) == codec_work_prefix(s,g,n)
        + tag_value(s,g.counts,n as int,g.le) + storage_bytes(g,g.tiled,n as int),
{ reveal_with_fuel(codec_work_prefix,1); }

proof fn codec_work_monotone(s: Seq<u8>, g: Geometry, n: nat, m: nat)
    requires n <= m,
        forall|j: int| 0 <= j < m ==> segment_valid(s,g,g.tiled,j),
    ensures 0 <= codec_work_prefix(s,g,n) <= codec_work_prefix(s,g,m),
    decreases m,
{
    reveal_with_fuel(codec_work_prefix,1);
    if m > 0 {
        if n < m { codec_work_monotone(s,g,n,(m-1) as nat); }
        else { codec_work_monotone(s,g,(m-1) as nat,(m-1) as nat); }
        assert(segment_valid(s,g,g.tiled,m as int-1));
        reveal(segment_valid);
        assert(storage_bytes(g,g.tiled,m as int-1) >= 0) by(nonlinear_arith)
            requires g.bilevel ==> 0 <= g.packed_row_bytes()*stored_rows(g,g.tiled,m as int-1),
                !g.bilevel ==> 0 <= g.block_width as int*stored_rows(g,g.tiled,m as int-1)*g.channels;
    }
}

#[verifier::opaque]
spec fn profile_layout(s: Seq<u8>, d: Directory, le: bool, limits: Limits) -> bool {
    let tiled = lookup(s, d, 324, le).is_some();
    let g = profile_geometry(s, d, le);
    &&& 0 < g.row_bytes() <= usize::MAX
    &&& g.positive()
    &&& g.across as int == ((g.width as int - 1) / (g.block_width as int)) + 1
    &&& (if tiled {
        &&& lookup(s, d, 273, le).is_none() && lookup(s, d, 279, le).is_none()
        &&& scalar_valid(s, d, 322, le) && scalar_valid(s, d, 323, le)
        &&& 0 < scalar_model(s, d, 322, 0, le) <= usize::MAX
        &&& 0 < scalar_model(s, d, 323, 0, le) <= usize::MAX
        &&& g.block_width % 16 == 0 && g.block_height % 16 == 0
    } else {
        &&& lookup(s, d, 322, le).is_none() && lookup(s, d, 323, le).is_none()
        &&& lookup(s, d, 325, le).is_none()
        &&& scalar_valid(s, d, 278, le)
        &&& 0 < scalar_model(s, d, 278, 4294967295, le) <= usize::MAX
    })
    &&& lookup(s, d, if tiled { 324 } else { 273 }, le).is_some()
    &&& lookup(s, d, if tiled { 325 } else { 279 }, le).is_some()
    &&& 0 <= segment_count(g) <= limits.max_segments
    &&& g.offsets.count == segment_count(g) && g.counts.count == segment_count(g)
    &&& (forall|j: int| 0 <= j < segment_count(g) ==> segment_valid(s, g, tiled, j))
    &&& 0 <= decode_base(g) <= usize::MAX
    &&& ((g.packbits || g.lzw) ==> 0 <= decode_base(g)
        + codec_work_prefix(s,g,segment_count(g) as nat) <= usize::MAX)
}

proof fn quotient_remainder(n: int, d: int)
    requires 0 <= n, 0 < d,
    ensures 0 <= n / d, 0 <= n % d < d, n == d * (n / d) + n % d,
{
    vstd::arithmetic::div_mod::lemma_div_basics_4(n, d);
    vstd::arithmetic::div_mod::lemma_mod_pos_bound(n, d);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod(n, d);
}

// Derive every sample's addressability from whole-segment storage, rather
// than including successful sample preflight as an input-validity assumption.
proof fn segment_covers_sample(s: Seq<u8>, g: Geometry, tiled: bool, k: int)
    requires s.len() <= usize::MAX, g.positive(), g.tiled == tiled,
        g.across as int == (g.width as int - 1) / (g.block_width as int) + 1,
        !tiled ==> g.block_width == g.width,
        segment_count(g) <= usize::MAX,
        0 <= k < g.width as int * g.height as int * g.channels as int,
        forall|j: int| 0 <= j < segment_count(g) ==> segment_valid(s, g, tiled, j),
    ensures sample_valid(s, g, k),
{
    let w = g.width as int; let h = g.height as int; let ch = g.channels as int;
    let bw = g.block_width as int; let bh = g.block_height as int;
    let a = g.across as int;
    let p = k / ch; let c = k % ch;
    quotient_remainder(k, ch);
    assert(0 <= p < w * h) by(nonlinear_arith)
        requires 0 < w, 0 < h, 0 < ch, 0 <= p, 0 <= c < ch,
            k == ch * p + c, 0 <= k < w * h * ch;
    let x = p % w; let y = p / w;
    quotient_remainder(p, w);
    assert(0 <= y < h) by(nonlinear_arith)
        requires 0 < w, 0 <= x < w, 0 <= y, p == w * y + x, 0 <= p < w * h;
    let bx = x / bw; let by = y / bh;
    let lx = x % bw; let ly = y % bh;
    quotient_remainder(x, bw); quotient_remainder(y, bh);
    vstd::arithmetic::div_mod::lemma_div_is_ordered(x, w - 1, bw);
    vstd::arithmetic::div_mod::lemma_div_is_ordered(y, h - 1, bh);
    let down = (h - 1) / bh + 1;
    let j = by * a + bx;
    assert(0 <= j < a * down) by(nonlinear_arith)
        requires 0 < a, 0 <= bx < a, 0 <= by < down, j == by * a + bx;
    assert(j == g.segment(k));
    assert(segment_valid(s, g, tiled, j));
    reveal(segment_valid);
    if !tiled {
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(w - 1, w, 0, w - 1);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(x, w, 0, x);
        assert(bw == w);
        assert(a == 1);
        assert(bx == 0);
        assert(j == by) by(nonlinear_arith)
            requires a == 1, bx == 0, j == by * a + bx;
    }
    let rows = stored_rows(g, tiled, j);
    assert(0 <= ly < rows) by(nonlinear_arith)
        requires 0 <= ly < bh, 0 <= y < h, y == bh * by + ly,
            tiled ==> rows == bh,
            !tiled ==> j == by,
            !tiled ==> rows == (if h - j * bh < bh { h - j * bh } else { bh });
    let count = tag_value(s, g.counts, j, g.le);
    let start = tag_value(s, g.offsets, j, g.le);
    assert(0 <= by * a <= j) by(nonlinear_arith)
        requires 0 <= by, 0 < a, 0 <= bx, j == by * a + bx;
    if g.bilevel {
        assert(ch == 1);
        let stride = g.packed_row_bytes();
        vstd::arithmetic::div_mod::lemma_div_is_ordered(lx, bw - 1, 8);
        quotient_remainder(lx, 8);
        let col = lx / 8;
        let row = ly * stride;
        let local = row + col;
        assert(0 <= row <= local < stride * rows) by(nonlinear_arith)
            requires 0 < stride, 0 <= col < stride, 0 <= ly < rows,
                row == ly * stride, local == row + col;
        assert(local == g.local(k));
        assert(0 <= local < storage_bytes(g, tiled, j));
        if !(g.packbits || g.lzw) { assert(0 <= local < count && count <= usize::MAX); }
    } else {
        let row = ly * bw;
        let cell = row + lx;
        let base = cell * ch;
        let local = base + c;
        assert(0 <= row <= cell && cell <= base && base <= local && local < bw * rows * ch) by(nonlinear_arith)
            requires 0 < bw, 0 < ch, 0 <= ly < rows, 0 <= lx < bw, 0 <= c < ch,
                row == ly * bw, cell == row + lx, base == cell * ch, local == base + c;
        assert(local == g.local(k));
        assert(0 <= local < storage_bytes(g, tiled, j));
        if !(g.packbits || g.lzw) { assert(0 <= local < count && count <= usize::MAX); }
    }
    assert(address_arithmetic_fits(g, k));
}

spec fn sample_slot(g: Geometry, k: int) -> int {
    if g.bilevel { ((k / (g.channels as int)) % (g.width as int) % (g.block_width as int)) % 8 }
    else { 0 }
}

spec fn target_x(g: Geometry, j: int, i: int, b: int) -> int {
    (j % (g.across as int)) * g.block_width
        + if g.bilevel { (i % g.row_bytes()) * 8 + b }
          else { (i % g.row_bytes()) / (g.channels as int) }
}

spec fn target_y(g: Geometry, j: int, i: int) -> int {
    (j / (g.across as int)) * g.block_height + i / g.row_bytes()
}

spec fn target_channel(g: Geometry, i: int) -> int {
    if g.bilevel { 0 } else { (i % g.row_bytes()) % (g.channels as int) }
}

spec fn target_index(g: Geometry, j: int, i: int, b: int) -> int {
    (target_y(g, j, i) * g.width + target_x(g, j, i, b)) * g.channels + target_channel(g, i)
}

spec fn target_valid(g: Geometry, j: int, i: int, b: int) -> bool {
    &&& g.positive() && j >= 0 && i >= 0
    &&& 0 <= b < (if g.bilevel { 8int } else { 1int })
    &&& (if g.bilevel { (i % g.row_bytes()) * 8 + b } else { (i % g.row_bytes()) / (g.channels as int) }) < g.block_width
    &&& i / g.row_bytes() < g.block_height
    &&& 0 <= target_x(g, j, i, b) < g.width
    &&& 0 <= target_y(g, j, i) < g.height
}

// The coordinate model has a unique output sample for each visible storage
// byte/bit. Padding positions deliberately have no output target.
proof fn sample_maps_to_target(g: Geometry, k: int)
    requires g.positive(),
        g.across as int == (g.width as int - 1) / (g.block_width as int) + 1,
        0 <= k < g.width as int * g.height * g.channels,
    ensures target_valid(g, g.segment(k), g.local(k), sample_slot(g, k)),
        target_index(g, g.segment(k), g.local(k), sample_slot(g, k)) == k,
        0 <= sample_slot(g, k) < (if g.bilevel { 8int } else { 1int }),
{
    let w = g.width as int; let h = g.height as int; let ch = g.channels as int;
    let bw = g.block_width as int; let bh = g.block_height as int;
    let a = g.across as int; let stride = g.row_bytes();
    let p = k / ch; let c = k % ch;
    quotient_remainder(k, ch);
    assert(0 <= p < w * h) by(nonlinear_arith)
        requires 0 < ch, 0 <= p, 0 <= c < ch, k == ch*p+c, 0 <= k < w*h*ch;
    let x = p % w; let y = p / w;
    quotient_remainder(p, w);
    assert(0 <= y < h) by(nonlinear_arith)
        requires 0 < w, 0 <= y, 0 <= x < w, p == w*y+x, p < w*h;
    let bx = x / bw; let by = y / bh;
    let lx = x % bw; let ly = y % bh;
    quotient_remainder(x, bw); quotient_remainder(y, bh);
    vstd::arithmetic::div_mod::lemma_div_is_ordered(x, w-1, bw);
    let j = by*a+bx;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(j, a, by, bx);
    assert(j == g.segment(k));
    let i = g.local(k);
    if g.bilevel {
        assert(ch == 1 && c == 0);
        quotient_remainder(lx, 8);
        vstd::arithmetic::div_mod::lemma_div_is_ordered(lx, bw-1, 8);
        assert(i == ly*stride+lx/8);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(i, stride, ly, lx/8);
    } else {
        let col = lx*ch+c;
        assert(stride > 0 && 0 <= col < stride) by(nonlinear_arith)
            requires stride == bw*ch, 0 < bw, 0 < ch, 0 <= lx < bw, 0 <= c < ch, col == lx*ch+c;
        assert(i == ly*stride+col) by(nonlinear_arith)
            requires i == (ly*bw+lx)*ch+c, stride == bw*ch, col == lx*ch+c;
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(i, stride, ly, col);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(col, ch, lx, c);
    }
    assert(i/stride == ly);
    assert(j/a == by);
    assert(j%a == bx);
    if g.bilevel {
        assert(i%stride == lx/8);
        assert(sample_slot(g,k) == lx%8);
        assert((i%stride)*8+sample_slot(g,k) == lx) by(nonlinear_arith)
            requires i%stride == lx/8, sample_slot(g,k) == lx%8,
                lx == 8*(lx/8)+lx%8;
    } else {
        assert((i%stride)/ch == lx);
    }
    assert(target_x(g, j, i, sample_slot(g, k)) == x) by(nonlinear_arith)
        requires target_x(g,j,i,sample_slot(g,k)) == bx*bw+lx, x == bw*bx+lx;
    assert(target_y(g, j, i) == y) by(nonlinear_arith)
        requires target_y(g,j,i) == by*bh+ly, y == bh*by+ly;
    assert(target_channel(g, i) == c);
    assert(target_index(g, j, i, sample_slot(g, k)) == k) by(nonlinear_arith)
        requires target_index(g,j,i,sample_slot(g,k)) == (y*w+x)*ch+c,
            p == w*y+x, k == ch*p+c;
}

proof fn target_maps_to_sample(g: Geometry, j: int, i: int, b: int)
    requires target_valid(g, j, i, b),
        g.across as int == (g.width as int - 1) / (g.block_width as int) + 1,
    ensures 0 <= target_index(g,j,i,b) < g.width as int * g.height * g.channels,
        g.segment(target_index(g,j,i,b)) == j,
        g.local(target_index(g,j,i,b)) == i,
        sample_slot(g,target_index(g,j,i,b)) == b,
{
    let w=g.width as int; let h=g.height as int; let ch=g.channels as int;
    let bw=g.block_width as int; let bh=g.block_height as int; let a=g.across as int;
    let stride=g.row_bytes();
    assert(stride > 0) by(nonlinear_arith)
        requires g.bilevel ==> stride == (bw-1)/8+1, !g.bilevel ==> stride == bw*ch,
            bw > 0, ch > 0;
    quotient_remainder(j,a); quotient_remainder(i,stride);
    let row=i/stride;
    let col=if g.bilevel { (i%stride)*8+b } else { (i%stride)/ch };
    let c=target_channel(g,i);
    if g.bilevel {
        assert(ch == 1 && c == 0);
        vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(col,8,i%stride,b);
    } else {
        quotient_remainder(i%stride,ch);
        assert(b == 0);
    }
    let x=target_x(g,j,i,b); let y=target_y(g,j,i);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(x,bw,j%a,col);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(y,bh,j/a,row);
    let p=y*w+x;
    let k=p*ch+c;
    assert(0 <= k < w*h*ch) by(nonlinear_arith)
        requires 0 < w, 0 < h, 0 < ch, 0 <= x < w, 0 <= y < h, 0 <= c < ch,
            p == y*w+x, k == p*ch+c;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k,ch,p,c);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(p,w,y,x);
    if !g.bilevel {
        assert(g.local(k) == i) by(nonlinear_arith)
            requires g.local(k) == (row*bw+col)*ch+c,
                stride == bw*ch, i == row*stride+i%stride, i%stride == col*ch+c;
    }
}

#[inline]
fn storage_target(g: Geometry, j: usize, i: usize, b: usize, stride: usize) -> (r: Option<usize>)
    requires g.positive(), stride as int == g.row_bytes(),
        g.width as int * g.height * g.channels <= usize::MAX,
    ensures r.is_some() == target_valid(g,j as int,i as int,b as int),
        r.is_some() ==> r.unwrap() == target_index(g,j as int,i as int,b as int),
        r.is_some() ==> r.unwrap() < g.width as int * g.height * g.channels,
{
    if b >= if g.bilevel { 8 } else { 1 } { return None; }
    assert(stride > 0) by(nonlinear_arith)
        requires g.block_width > 0, g.channels > 0,
            g.bilevel ==> stride as int == (g.block_width as int-1)/8+1,
            !g.bilevel ==> stride as int == g.block_width as int*g.channels;
    let row=i/stride;
    if row >= g.block_height { return None; }
    // Full-width eight-bit segments have no horizontal padding. Avoid
    // reconstructing pixel/channel coordinates for every decoded byte.
    if !g.bilevel && g.block_width==g.width && g.across==1 {
        reveal(target_y);reveal(target_x);reveal(target_channel);reveal(target_index);reveal(target_valid);
        assert(g.row_bytes()==stride as int);
        proof {vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(j as int,1,j as int,0);}
        assert(j as int/(g.across as int)==j as int);
        assert(i as int/g.row_bytes()==row as int);
        assert(target_y(g,j as int,i as int)==(j as int/(g.across as int))*g.block_height+i as int/g.row_bytes());
        assert(target_y(g,j as int,i as int)==j as int*g.block_height+row);
        assert(target_x(g,j as int,i as int,b as int)==(i as int%(stride as int))/(g.channels as int));
        let base_y=match mul(j,g.block_height) {Ok(v)=>v,Err(_)=>return None};
        if base_y>=g.height || row>=g.height-base_y {return None;}
        proof {
            quotient_remainder(i as int%(stride as int),g.channels as int);
            assert((i as int%(stride as int))/(g.channels as int)<g.width) by(nonlinear_arith)
                requires 0<=i as int%(stride as int)<stride,
                    stride as int==g.width as int*g.channels,g.width>0,g.channels>0,
                    i as int%(stride as int)==g.channels as int*((i as int%(stride as int))/(g.channels as int))+(i as int%(stride as int))%(g.channels as int),
                    0<=(i as int%(stride as int))%(g.channels as int)<g.channels;
            assert(0<=(base_y as int+row)*stride+i%stride<g.width as int*g.height*g.channels) by(nonlinear_arith)
                requires 0<=base_y as int+row<g.height,0<=i%stride<stride,stride>0,
                    stride as int==g.width as int*g.channels;
            assert(target_channel(g,i as int)==(i%stride)%g.channels);
            assert(target_index(g,j as int,i as int,b as int)==((base_y as int+row)*g.width+(i%stride)/g.channels)*g.channels+(i%stride)%g.channels);
            assert(target_index(g,j as int,i as int,b as int)==(base_y as int+row)*stride+i%stride) by(nonlinear_arith)
                requires target_index(g,j as int,i as int,b as int)==((base_y as int+row)*g.width+(i%stride)/g.channels)*g.channels+(i%stride)%g.channels,
                    stride as int==g.width as int*g.channels,
                    i%stride==g.channels as int*((i%stride)/g.channels)+(i%stride)%g.channels;
        }
        return Some((base_y+row)*stride+i%stride);
    }
    let col=if g.bilevel {
        let base=match mul(i%stride,8) { Ok(v)=>v, Err(_)=>return None };
        match add(base,b) { Ok(v)=>v, Err(_)=>return None }
    } else { (i%stride)/g.channels };
    if col >= g.block_width { return None; }
    let base_x=match mul(j%g.across,g.block_width) { Ok(v)=>v, Err(_)=>return None };
    let base_y=match mul(j/g.across,g.block_height) { Ok(v)=>v, Err(_)=>return None };
    if base_x >= g.width || col >= g.width-base_x { return None; }
    if base_y >= g.height || row >= g.height-base_y { return None; }
    let x=base_x+col; let y=base_y+row;
    let c=if g.bilevel { 0 } else { (i%stride)%g.channels };
    assert((y as int*g.width+x)*g.channels+c < g.width as int*g.height*g.channels) by(nonlinear_arith)
        requires 0 <= x < g.width, 0 <= y < g.height, 0 <= c < g.channels,
            g.width > 0, g.channels > 0;
    assert(0 <= y as int*g.width <= y as int*g.width+x <= (y as int*g.width+x)*g.channels+c) by(nonlinear_arith)
        requires y >= 0, g.width > 0, x >= 0, g.channels >= 1, c >= 0;
    Some((y*g.width+x)*g.channels+c)
}

spec fn normalized(v: u8, g: Geometry, k: int) -> u8 {
    let value = if g.bilevel {
        if (v as int / power2(g.bit_position(k))) % 2 == 0 { 0u8 } else { 255u8 }
    } else { v };
    if g.white && g.alpha==0 { (255-value) as u8 } else { value }
}

// One coordinate calculation covers all visible bits of a storage byte.
#[inline(always)]
fn bilevel_span(g:Geometry,j:usize,i:usize,stride:usize) -> (r:(usize,usize))
    requires g.positive(),g.bilevel,stride as int==g.row_bytes(),
        g.width as int*g.height*g.channels<=usize::MAX,
    ensures r.1<=8,r.0 as int+r.1<=g.width as int*g.height*g.channels,
        forall|b:int| 0<=b<8 ==> (target_valid(g,j as int,i as int,b)==(b<r.1)),
        forall|b:int| 0<=b<r.1 ==> target_index(g,j as int,i as int,b)==r.0 as int+b,
{
    let base=match storage_target(g,j,i,0,stride) {
        Some(k)=>k,
        None=>{
            assert forall|b:int| 0<=b<8 implies !target_valid(g,j as int,i as int,b) by {
                assert(target_x(g,j as int,i as int,b)==target_x(g,j as int,i as int,0)+b);
                assert(target_x(g,j as int,i as int,0)>=0) by(nonlinear_arith)
                    requires g.across>0,g.block_width>0,0<=j as int%(g.across as int),0<=i as int%(stride as int),
                        target_x(g,j as int,i as int,0)==(j as int%(g.across as int))*g.block_width+(i as int%(stride as int))*8;
            }
            return (0,0);
        }
    };
    assert(target_valid(g,j as int,i as int,0));
    let ghost x=target_x(g,j as int,i as int,0);let ghost y=target_y(g,j as int,i as int);
    proof {vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(base as int,g.width as int,y,x);}
    let col=(i%stride)*8;
    let room=g.width-base%g.width;
    let tile_room=g.block_width-col;
    let n=if room<8 {room} else {8};
    let n=if tile_room<n {tile_room} else {n};
    assert(base as int+n<=g.width as int*g.height*g.channels) by(nonlinear_arith)
        requires base as int==y*g.width+x,0<=y<g.height,0<=x<g.width,0<n<=g.width as int-x,g.width>0,g.channels==1;
    assert forall|b:int| 0<=b<8 implies target_valid(g,j as int,i as int,b)==(b<n) by {
        assert(target_x(g,j as int,i as int,b)==x+b);
    }
    assert forall|b:int| 0<=b<n implies target_index(g,j as int,i as int,b)==base as int+b by {
        assert(target_x(g,j as int,i as int,b)==x+b);
    }
    (base,n)
}

#[inline(always)]
fn scatter_byte(g: Geometry, j: usize, i: usize, stride: usize, byte: u8, output: &mut [u8])
    requires g.positive(), stride as int == g.row_bytes(),
        g.across as int == (g.width as int-1)/(g.block_width as int)+1,
        g.width as int*g.height*g.channels <= old(output).len(),
    ensures final(output).len() == old(output).len(),
        forall|k: int| 0 <= k < g.width as int*g.height*g.channels ==>
            final(output)@[k] == (if g.segment(k) == j && g.local(k) == i {
                normalized(byte,g,k)
            } else { old(output)@[k] }),
        forall|k: int| g.width as int*g.height*g.channels <= k < old(output).len() ==>
            final(output)@[k] == old(output)@[k],
{
    let slots=if g.bilevel { 8usize } else { 1usize };
    let (span_start,span_len)=if g.bilevel {bilevel_span(g,j,i,stride)} else {(0usize,0usize)};
    let mut b=0usize;
    while b<slots
        invariant g.bilevel ==> span_len<=8 && span_start as int+span_len<=g.width as int*g.height*g.channels,
            g.bilevel ==> forall|q:int| 0<=q<8 ==> (target_valid(g,j as int,i as int,q)==(q<span_len)),
            g.bilevel ==> forall|q:int| 0<=q<span_len ==> target_index(g,j as int,i as int,q)==span_start as int+q,
            b<=slots, slots == if g.bilevel { 8usize } else { 1usize },
            g.positive(), stride as int == g.row_bytes(),
            g.across as int == (g.width as int-1)/(g.block_width as int)+1,
            output.len() == old(output).len(),
            g.width as int*g.height*g.channels <= output.len(),
            forall|k: int| 0 <= k < g.width as int*g.height*g.channels ==>
                #[trigger] output@[k] == (if g.segment(k) == j && g.local(k) == i && sample_slot(g,k) < b {
                    normalized(byte,g,k)
                } else { old(output)@[k] }),
            forall|k: int| g.width as int*g.height*g.channels <= k < old(output).len() ==>
                output@[k] == old(output)@[k],
        decreases slots-b,
    {
        let ghost prior_output=output@;
        let ghost prior_b=b;
        let target=if g.bilevel {if b<span_len {Some(span_start+b)} else {None}}
            else {storage_target(g,j,i,b,stride)};
        assert(target.is_some()==target_valid(g,j as int,i as int,b as int));
        assert(target.is_some() ==> target.unwrap()==target_index(g,j as int,i as int,b as int));
        proof {
            if target.is_some() { target_maps_to_sample(g,j as int,i as int,b as int); }
            assert forall|k: int| 0 <= k < g.width as int*g.height*g.channels
                && g.segment(k) == j && g.local(k) == i && sample_slot(g,k) == b
                implies target == Some(k as usize) by {
                sample_maps_to_target(g,k);
            }
        }
        if let Some(k)=target {
            let value=if g.bilevel {
                let shift=if g.lsb { b } else { 7-b };
                let digit=((byte as u32)>>(shift as u32))&1;
                proof {
                    bit_window::powers(shift as u32);
                    assert(deflate_bits::pow(shift as int)==power2(shift as int));
                    assert(digit==((byte as u32)/(1u32<<(shift as u32)))%2) by(bit_vector)
                        requires shift<8,digit==((byte as u32)>>(shift as u32))&1;
                }
                if digit==0 {0u8} else {255u8}
            } else { byte };
            let final_value=if g.white && g.alpha==0 {255-value} else {value};
            assert(final_value == normalized(byte,g,k as int));
            output[k]=final_value;
        }
        b=b+1;
        proof {
            assert forall|k: int| 0 <= k < g.width as int*g.height*g.channels implies
                #[trigger] output@[k] == (if g.segment(k) == j && g.local(k) == i && sample_slot(g,k) < b {
                    normalized(byte,g,k)
                } else { old(output)@[k] }) by {
                assert(prior_output[k] == (if g.segment(k) == j && g.local(k) == i && sample_slot(g,k) < prior_b {
                    normalized(byte,g,k)
                } else { old(output)@[k] }));
                if g.segment(k) == j && g.local(k) == i && sample_slot(g,k) == prior_b {
                    sample_maps_to_target(g,k);
                    assert(target == Some(k as usize));
                }
            }
        }
    }
    proof {
        assert forall|k: int| 0 <= k < g.width as int*g.height*g.channels
            implies 0 <= #[trigger] sample_slot(g,k) < slots by {
            sample_maps_to_target(g,k);
        }
    }
}

#[verifier::opaque]
pub closed spec fn supported_profile(s: Seq<u8>, page: usize, limits: Limits) -> bool {
    let le = s[0] == 73;
    let first = word32(s, 4, le);
    let off = nth_ifd(s, first, le, page as nat);
    let d = directory_model(s, off, le);
    &&& header_valid(s)
    &&& s.len() <= limits.max_input && page < limits.max_directories
    &&& (forall|p: nat| p <= page ==> {
        let at = #[trigger] nth_ifd(s, first, le, p);
        0 < at <= usize::MAX && directory_valid(s, at, le, limits)
    })
    &&& profile_fields(s, d, le, limits)
    &&& profile_layout(s, d, le, limits)
}

pub open spec fn inspection_work_bound(page: usize, limits: Limits) -> int {
    (page as int + 1) * (limits.max_tags as int + 1)
        + 19 * limits.max_tags + 4 + (limits.max_segments as int)*(limits.max_input as int+1) + limits.max_output
}

/// A conservative, proved sufficient inspection budget for the selected page
/// and limits. The bound may exceed usize even when a particular small file
/// would decode; in that case choose tighter limits or budget explicitly.
pub fn inspection_work_limit(page: usize, limits: &Limits) -> (r: Result<usize, Error>)
    ensures r.is_ok() == (inspection_work_bound(page, *limits) <= usize::MAX),
        r.is_ok() ==> r.unwrap() == inspection_work_bound(page, *limits),
{
    let ghost p = page as int;
    let ghost t = limits.max_tags as int;
    assert(p + 1 <= (p + 1) * (t + 1) && t + 1 <= (p + 1) * (t + 1)) by(nonlinear_arith)
        requires 0 <= p, 0 <= t;
    let dirs = mul(add(page, 1)?, add(limits.max_tags, 1)?)?;
    let tags = mul(19, limits.max_tags)?;
    let ghost ms=limits.max_segments as int;
    let ghost mi=limits.max_input as int;
    assert(0 <= ms*mi <= ms*mi+ms && ms*mi+ms == ms*(mi+1)) by(nonlinear_arith)
        requires ms >= 0, mi >= 0;
    let segments = add(mul(limits.max_segments, limits.max_input)?, limits.max_segments)?;
    add(add(add(add(dirs, tags)?, 4)?, segments)?, limits.max_output)
}

proof fn traversal_budget(initial: int, remaining: int, p: int, page: int,
    tags: int, tail: int)
    requires 0 <= p <= page, 0 <= tags, 0 <= tail,
        initial >= (page + 1) * (tags + 1) + tail,
        initial - remaining <= p * (tags + 1),
    ensures remaining >= tags + 1 + tail,
{
    assert(remaining >= tags + 1 + tail) by(nonlinear_arith)
        requires 0 <= p <= page, 0 <= tags, 0 <= tail,
            initial >= (page + 1) * (tags + 1) + tail,
            initial - remaining <= p * (tags + 1);
}

// A concrete formal non-vacuity witness: a one-byte BlackIsZero strip, with
// default compression, rows-per-strip, and samples-per-pixel. Its bytes are
// also committed as a runtime regression fixture.
pub closed spec fn profile_witness_bytes(bilevel: bool) -> Seq<u8> {
    Seq::new(87, |i: int| {
        if i == 0 || i == 1 { 73u8 }
        else if i == 2 { 42u8 }
        else if i == 4 { 8u8 }
        else if i == 42 { if bilevel { 1u8 } else { 8u8 } }
        else if i == 8 || i == 46 { 6u8 }
        else if i == 11 || i == 14 || i == 18 || i == 22 || i == 23 || i == 26 || i == 30 || i == 35 || i == 38 || i == 47 || i == 50 || i == 54 || i == 59 || i == 62 || i == 71 || i == 74 || i == 78 { 1u8 }
        else if i == 12 || i == 24 || i == 36 || i == 48 { 3u8 }
        else if i == 34 { 2u8 }
        else if i == 58 { 17u8 }
        else if i == 60 || i == 72 { 4u8 }
        else if i == 66 { 86u8 }
        else if i == 70 { 23u8 }
        else if i == 86 { 123u8 }
        else { 0u8 }
    })
}

pub closed spec fn profile_witness_limits() -> Limits {
    Limits { max_input: 87, max_dimension: 1, max_output: 1,
        max_directories: 1, max_tags: 6, max_segments: 1 }
}

pub proof fn supported_profile_has_witness(bilevel: bool)
    ensures supported_profile(profile_witness_bytes(bilevel), 0, profile_witness_limits()),
        inspection_work_bound(0, profile_witness_limits()) == 214,
{
    reveal(alpha_field_valid);
    let s = profile_witness_bytes(bilevel);
    let limits = profile_witness_limits();
    let d = Directory { start: 10, count: 6, next: 0 };
    reveal(tag_valid);
    assert forall|i: int| 0 <= i < 6 implies #[trigger] tag_valid(s, 10 + 12 * i, true) by {
        if i == 0 {} else if i == 1 {} else if i == 2 {}
        else if i == 3 {} else if i == 4 {} else { assert(i == 5); }
    }
    assert forall|i: int| 0 <= i < 6 implies
        #[trigger] word16(s, 10 + 12 * i, true) == seq![256int, 257, 258, 262, 273, 279][i] by {
        if i == 0 {} else if i == 1 {} else if i == 2 {}
        else if i == 3 {} else if i == 4 {} else { assert(i == 5); }
    }
    lookup_at(s, d, 256, true, 0); lookup_at(s, d, 257, true, 1);
    lookup_at(s, d, 258, true, 2); lookup_at(s, d, 262, true, 3);
    lookup_at(s, d, 273, true, 4); lookup_at(s, d, 279, true, 5);
    assert forall|id: u16| id != 256 && id != 257 && id != 258 && id != 262 && id != 273 && id != 279
        implies #[trigger] lookup(s, d, id, true).is_none() by {
        assert forall|i: int| 0 <= i < 6 implies #[trigger] word16(s, 10 + 12 * i, true) != id by {
            if i == 0 {} else if i == 1 {} else if i == 2 {}
            else if i == 3 {} else if i == 4 {} else { assert(i == 5); }
        }
        lookup_missing(s, d, id, true);
    }
    reveal(profile_fields);
    reveal(profile_layout);
    reveal(segment_valid);
    assert(directory_valid(s, 8, true, limits));
    assert(bits_model(s, d, true) == if bilevel { 1int } else { 8int });
    let bit_tag = Tag { kind: 3, count: 1, data: 42 };
    assert(lookup(s, d, 258, true) == Some(bit_tag));
    assert forall|i: int| 0 <= i < 1 implies value_valid(s, bit_tag, i)
        && #[trigger] tag_value(s, bit_tag, i, true) == (if bilevel { 1int } else { 8int }) by {
        assert(i == 0);
    }
    assert(bits_supported(s, d, 1, true));
    assert(lookup(s, d, 277, true).is_none());
    assert(lookup(s, d, 266, true).is_none());
    assert(lookup(s, d, 259, true).is_none());
    assert(lookup(s, d, 274, true).is_none());
    assert(lookup(s, d, 284, true).is_none());
    assert(lookup(s, d, 317, true).is_none());
    assert(lookup(s, d, 338, true).is_none());
    assert(lookup(s, d, 339, true).is_none());
    assert(scalar_model(s, d, 256, 0, true) == 1);
    assert(scalar_model(s, d, 257, 0, true) == 1);
    assert(scalar_model(s, d, 277, 1, true) == 1);
    assert(scalar_model(s, d, 262, 999, true) == 1);
    assert(scalar_valid(s, d, 256, true) && scalar_valid(s, d, 257, true));
    assert(scalar_valid(s, d, 277, true) && scalar_valid(s, d, 262, true));
    assert(scalar_valid(s, d, 259, true) && scalar_model(s, d, 259, 1, true) == 1);
    assert(scalar_valid(s, d, 274, true) && scalar_model(s, d, 274, 1, true) == 1);
    assert(scalar_valid(s, d, 284, true) && scalar_model(s, d, 284, 1, true) == 1);
    assert(scalar_valid(s, d, 266, true) && scalar_model(s, d, 266, 1, true) == 1);
    assert(scalar_valid(s, d, 317, true) && scalar_model(s, d, 317, 1, true) == 1);
    assert(lookup(s, d, 338, true).is_none() && lookup(s, d, 339, true).is_none());
    assert(0 < scalar_model(s,d,256,0,true) <= limits.max_dimension);
    assert(0 < scalar_model(s,d,257,0,true) <= limits.max_dimension);
    assert(0 <= scalar_model(s,d,256,0,true)*scalar_model(s,d,257,0,true) <= usize::MAX);
    assert(0 <= scalar_model(s,d,256,0,true)*scalar_model(s,d,257,0,true)*scalar_model(s,d,277,1,true) <= limits.max_output);
    assert(bits_supported(s,d,scalar_model(s,d,277,1,true),true));
    assert(match lookup(s,d,339,true) { None=>true,Some(t)=>array_of(s,t,1,1,true) });
    assert(profile_fields(s, d, true, limits));
    assert(lookup(s, d, 324, true).is_none());
    assert(lookup(s, d, 278, true).is_none());
    assert(lookup(s, d, 277, true).is_none());
    let g = profile_geometry(s, d, true);
    assert(g.width == 1 && g.height == 1 && g.channels == 1);
    assert(g.block_width == 1 && g.block_height == 4294967295 && g.across == 1);
    assert(g.offsets == (Tag { kind: 4, count: 1, data: 66 }));
    assert(g.counts == (Tag { kind: 4, count: 1, data: 78 }));
    assert(g.le && !g.white);
    assert(g == (Geometry { width: 1, height: 1, channels: 1, alpha:0,
        block_width: 1, block_height: 4294967295, across: 1,
        offsets: Tag { kind: 4, count: 1, data: 66 },
        counts: Tag { kind: 4, count: 1, data: 78 }, le: true, white: false, bilevel, lsb: false, packbits: false, lzw: false, tiled: false }));
    assert(segment_count(g) == 1);
    assert(segment_valid(s, g, false, 0));
    assert forall|j: int| 0 <= j < segment_count(g) implies segment_valid(s, g, false, j) by {
        assert(j == 0);
    }
    assert(profile_layout(s, d, true, limits));
    reveal(supported_profile);
}

// Concrete Gray+A/RGBA witnesses for both ExtraSamples alpha conventions.
pub closed spec fn alpha_witness_bytes(rgb:bool,associated:bool) -> Seq<u8> {
    Seq::new(122,|i:int| {
        if i==38 { if rgb {4u8} else {2u8} }
        else if i==42 { if rgb {110u8} else {8u8} }
        else if i==44 { if rgb {0u8} else {8u8} }
        else if i==54 { if rgb {2u8} else {1u8} }
        else if i==78 { if rgb {4u8} else {2u8} }
        else if i==90 { if rgb {4u8} else {2u8} }
        else if i==102 { if associated {1u8} else {2u8} }
        else if i==119 { if rgb {64u8} else {128u8} }
        else if i==11 || i==14 || i==18 || i==22 || i==23 || i==26 || i==30 || i==35 || i==47 || i==50 || i==59 || i==62 || i==71 || i==74 || i==83 || i==86 || i==95 || i==98 { 1u8 }
        else if i==34 { 2u8 }
        else if i==12 || i==24 || i==36 || i==48 || i==72 || i==96 { 3u8 }
        else if i==60 || i==84 { 4u8 }
        else if i==46 { 6u8 }
        else if i==4 || i==8 || i==110 || i==112 || i==114 || i==116 { 8u8 }
        else if i==58 { 17u8 }
        else if i==70 { 21u8 }
        else if i==82 { 23u8 }
        else if i==118 { 32u8 }
        else if i==2 { 42u8 }
        else if i==0 || i==1 { 73u8 }
        else if i==94 { 82u8 }
        else if i==120 { 96u8 }
        else if i==66 { 118u8 }
        else if i==121 { 128u8 }
        else {0u8}
    })
}
pub closed spec fn alpha_witness_limits() -> Limits {
    Limits {max_input:122,max_dimension:1,max_output:4,max_directories:1,max_tags:8,max_segments:1}
}
pub proof fn alpha_profile_has_witness(rgb:bool,associated:bool)
    ensures supported_profile(alpha_witness_bytes(rgb,associated),0,alpha_witness_limits()),
        inspection_work_bound(0,alpha_witness_limits())==292,
{
    reveal(alpha_field_valid);
    let s=alpha_witness_bytes(rgb,associated);
    let limits=alpha_witness_limits();
    let d=Directory {start:10,count:8,next:0};
    let ch=if rgb {4int} else {2int};
    let alpha=if associated {1usize} else {2usize};
    reveal(tag_valid);
    assert forall|i:int| 0<=i<8 implies #[trigger] tag_valid(s,10+12*i,true) by {
        if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {}
        else if i==4 {} else if i==5 {} else if i==6 {} else {assert(i==7);}
    }
    assert forall|i:int| 0<=i<8 implies
        #[trigger] word16(s,10+12*i,true)==seq![256int,257,258,262,273,277,279,338][i] by {
        if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {}
        else if i==4 {} else if i==5 {} else if i==6 {} else {assert(i==7);}
    }
    lookup_at(s,d,256,true,0);lookup_at(s,d,257,true,1);
    lookup_at(s,d,258,true,2);lookup_at(s,d,262,true,3);
    lookup_at(s,d,273,true,4);lookup_at(s,d,277,true,5);
    lookup_at(s,d,279,true,6);lookup_at(s,d,338,true,7);
    assert forall|id:u16| id!=256 && id!=257 && id!=258 && id!=262 && id!=273 && id!=277 && id!=279 && id!=338
        implies #[trigger] lookup(s,d,id,true).is_none() by {
        assert forall|i:int| 0<=i<8 implies #[trigger] word16(s,10+12*i,true)!=id by {
            if i==0 {} else if i==1 {} else if i==2 {} else if i==3 {}
            else if i==4 {} else if i==5 {} else if i==6 {} else {assert(i==7);}
        }
        lookup_missing(s,d,id,true);
    }
    reveal(profile_fields);reveal(profile_layout);reveal(segment_valid);
    assert(directory_valid(s,8,true,limits));
    let bit_tag=Tag {kind:3,count:ch as usize,data:if rgb {110usize} else {42usize}};
    assert(lookup(s,d,258,true)==Some(bit_tag)) by {
        reveal(tag_model);reveal(field_size);
        assert(ch==2 || ch==4);
        assert(ch*2==if rgb {8int} else {4int}) by(nonlinear_arith) requires ch==if rgb {4int} else {2int};
        assert(word16(s,36,true)==3);
        assert(word32(s,38,true)==ch);
        if rgb { assert(word32(s,42,true)==110); }
        assert(tag_model(s,34,true)==bit_tag);
    }
    assert forall|i:int| 0<=i<ch implies value_valid(s,bit_tag,i)
        && #[trigger] tag_value(s,bit_tag,i,true)==8 by {
        if i==0 {} else if i==1 {} else if i==2 {} else {assert(i==3);}
    }
    assert(bits_supported(s,d,ch,true));
    assert(scalar_model(s,d,256,0,true)==1);
    assert(scalar_model(s,d,257,0,true)==1);
    assert(scalar_model(s,d,277,1,true)==ch);
    assert(scalar_model(s,d,262,999,true)==if rgb {2int} else {1int});
    assert(scalar_model(s,d,338,0,true)==alpha);
    assert(alpha_field_valid(s,d,true));
    assert(scalar_valid(s,d,256,true) && scalar_valid(s,d,257,true));
    assert(scalar_valid(s,d,277,true) && scalar_valid(s,d,262,true));
    assert(lookup(s,d,259,true).is_none());
    assert(lookup(s,d,274,true).is_none());
    assert(lookup(s,d,284,true).is_none());
    assert(lookup(s,d,266,true).is_none());
    assert(lookup(s,d,317,true).is_none());
    assert(scalar_valid(s,d,259,true) && scalar_model(s,d,259,1,true)==1);
    assert(scalar_valid(s,d,274,true) && scalar_model(s,d,274,1,true)==1);
    assert(scalar_valid(s,d,284,true) && scalar_model(s,d,284,1,true)==1);
    assert(scalar_valid(s,d,266,true) && scalar_model(s,d,266,1,true)==1);
    assert(scalar_valid(s,d,317,true) && scalar_model(s,d,317,1,true)==1);
    assert(lookup(s,d,339,true).is_none());
    assert(0<scalar_model(s,d,256,0,true)<=limits.max_dimension);
    assert(0<scalar_model(s,d,257,0,true)<=limits.max_dimension);
    assert(0<=scalar_model(s,d,256,0,true)*scalar_model(s,d,257,0,true)<=usize::MAX);
    assert(0<=scalar_model(s,d,256,0,true)*scalar_model(s,d,257,0,true)*scalar_model(s,d,277,1,true)<=limits.max_output);
    assert(profile_fields(s,d,true,limits));
    assert(lookup(s,d,324,true).is_none());assert(lookup(s,d,278,true).is_none());
    let g=profile_geometry(s,d,true);
    assert(g==(Geometry {width:1,height:1,channels:ch as usize,alpha,
        block_width:1,block_height:4294967295,across:1,
        offsets:Tag {kind:4,count:1,data:66},counts:Tag {kind:4,count:1,data:90},
        le:true,white:false,bilevel:false,lsb:false,packbits:false,lzw:false,tiled:false}));
    assert(segment_count(g)==1);
    assert(segment_valid(s,g,false,0));
    assert forall|j:int| 0<=j<segment_count(g) implies segment_valid(s,g,false,j) by {assert(j==0);}
    assert(profile_layout(s,d,true,limits));
    reveal(supported_profile);
}

pub closed spec fn packbits_witness_bytes(bilevel: bool) -> Seq<u8> {
    Seq::new(100, |i: int| {
        if i == 42 { if bilevel {1u8} else {8u8} }
        else if i == 11 || i == 14 || i == 18 || i == 22 || i == 23 || i == 26 || i == 30 || i == 35 || i == 38 || i == 47 || i == 50 || i == 59 || i == 62 || i == 66 || i == 71 || i == 74 || i == 83 || i == 86 { 1u8 }
        else if i == 34 || i == 90 { 2u8 }
        else if i == 12 || i == 24 || i == 36 || i == 46 || i == 48 || i == 60 { 3u8 }
        else if i == 72 || i == 84 { 4u8 }
        else if i == 54 { 5u8 }
        else if i == 58 { 6u8 }
        else if i == 8 { 7u8 }
        else if i == 4 { 8u8 }
        else if i == 70 { 17u8 }
        else if i == 82 { 23u8 }
        else if i == 2 { 42u8 }
        else if i == 0 || i == 1 { 73u8 }
        else if i == 78 { 98u8 }
        else if i == 99 { 123u8 }
        else if i == 55 { 128u8 }
        else {0u8}
    })
}

pub closed spec fn packbits_witness_limits() -> Limits {
    Limits { max_input:100, max_dimension:1, max_output:1, max_directories:1, max_tags:7, max_segments:1 }
}

pub proof fn packbits_profile_has_witness(bilevel: bool)
    ensures supported_profile(packbits_witness_bytes(bilevel), 0, packbits_witness_limits()),
        inspection_work_bound(0, packbits_witness_limits()) == 247,
{
    reveal(alpha_field_valid);
    let s = packbits_witness_bytes(bilevel);
    let limits = packbits_witness_limits();
    let d = Directory { start: 10, count: 7, next: 0 };
    reveal(tag_valid);
    assert forall|i: int| 0 <= i < 7 implies #[trigger] tag_valid(s, 10 + 12 * i, true) by {
        if i == 0 {} else if i == 1 {} else if i == 2 {}
        else if i == 3 {} else if i == 4 {} else if i == 5 {} else { assert(i == 6); }
    }
    assert forall|i: int| 0 <= i < 7 implies
        #[trigger] word16(s, 10 + 12 * i, true) == seq![256int, 257, 258, 259, 262, 273, 279][i] by {
        if i == 0 {} else if i == 1 {} else if i == 2 {}
        else if i == 3 {} else if i == 4 {} else if i == 5 {} else { assert(i == 6); }
    }
    lookup_at(s, d, 256, true, 0); lookup_at(s, d, 257, true, 1);
    lookup_at(s, d, 258, true, 2); lookup_at(s, d, 259, true, 3); lookup_at(s, d, 262, true, 4);
    lookup_at(s, d, 273, true, 5); lookup_at(s, d, 279, true, 6);
    assert forall|id: u16| id != 256 && id != 257 && id != 258 && id != 259 && id != 262 && id != 273 && id != 279
        implies #[trigger] lookup(s, d, id, true).is_none() by {
        assert forall|i: int| 0 <= i < 7 implies #[trigger] word16(s, 10 + 12 * i, true) != id by {
            if i == 0 {} else if i == 1 {} else if i == 2 {}
            else if i == 3 {} else if i == 4 {} else if i == 5 {} else { assert(i == 6); }
        }
        lookup_missing(s, d, id, true);
    }
    reveal(profile_fields);
    reveal(profile_layout);
    reveal(segment_valid);
    assert(directory_valid(s, 8, true, limits));
    assert(bits_model(s, d, true) == if bilevel { 1int } else { 8int });
    let bit_tag = Tag { kind: 3, count: 1, data: 42 };
    assert(lookup(s, d, 258, true) == Some(bit_tag));
    assert forall|i: int| 0 <= i < 1 implies value_valid(s, bit_tag, i)
        && #[trigger] tag_value(s, bit_tag, i, true) == (if bilevel { 1int } else { 8int }) by {
        assert(i == 0);
    }
    assert(bits_supported(s, d, 1, true));
    assert(lookup(s, d, 277, true).is_none());
    assert(lookup(s, d, 266, true).is_none());
    assert(lookup(s, d, 274, true).is_none());
    assert(lookup(s, d, 284, true).is_none());
    assert(lookup(s, d, 317, true).is_none());
    assert(lookup(s, d, 338, true).is_none());
    assert(lookup(s, d, 339, true).is_none());
    assert(scalar_model(s, d, 256, 0, true) == 1);
    assert(scalar_model(s, d, 257, 0, true) == 1);
    assert(scalar_model(s, d, 277, 1, true) == 1);
    assert(scalar_model(s, d, 262, 999, true) == 1);
    assert(scalar_valid(s, d, 256, true) && scalar_valid(s, d, 257, true));
    assert(scalar_valid(s, d, 277, true) && scalar_valid(s, d, 262, true));
    assert(scalar_valid(s, d, 259, true) && scalar_model(s, d, 259, 1, true) == 32773);
    assert(scalar_valid(s, d, 274, true) && scalar_model(s, d, 274, 1, true) == 1);
    assert(scalar_valid(s, d, 284, true) && scalar_model(s, d, 284, 1, true) == 1);
    assert(scalar_valid(s, d, 266, true) && scalar_model(s, d, 266, 1, true) == 1);
    assert(scalar_valid(s, d, 317, true) && scalar_model(s, d, 317, 1, true) == 1);
    assert(lookup(s, d, 338, true).is_none() && lookup(s, d, 339, true).is_none());
    assert(profile_fields(s, d, true, limits));
    assert(lookup(s, d, 324, true).is_none());
    assert(lookup(s, d, 278, true).is_none());
    assert(lookup(s, d, 277, true).is_none());
    let g = profile_geometry(s, d, true);
    assert(g.width == 1 && g.height == 1 && g.channels == 1);
    assert(g.block_width == 1 && g.block_height == 4294967295 && g.across == 1);
    assert(g.offsets == (Tag { kind: 4, count: 1, data: 78 }));
    assert(g.counts == (Tag { kind: 4, count: 1, data: 90 }));
    assert(g.le && !g.white);
    assert(g == (Geometry { width: 1, height: 1, channels: 1, alpha:0,
        block_width: 1, block_height: 4294967295, across: 1,
        offsets: Tag { kind: 4, count: 1, data: 78 },
        counts: Tag { kind: 4, count: 1, data: 90 }, le: true, white: false, bilevel, lsb: false, packbits: true, lzw: false, tiled: false }));
    assert(segment_count(g) == 1);
    reveal_with_fuel(pb_valid,2);
    assert(pb_valid(s,98,100,1,1));
    assert(segment_valid(s, g, false, 0));
    assert forall|j: int| 0 <= j < segment_count(g) implies segment_valid(s, g, false, j) by {
        assert(j == 0);
    }
    reveal_with_fuel(codec_work_prefix,2);
    assert(codec_work_prefix(s,g,1) == 3);
    assert(profile_layout(s, d, true, limits));
    reveal(supported_profile);
}

pub closed spec fn lzw_witness_bytes(bilevel: bool) -> Seq<u8> {
    lzw_variant_witness_bytes(bilevel,false)
}
pub closed spec fn lzw_legacy_witness_bytes(bilevel: bool) -> Seq<u8> {
    lzw_variant_witness_bytes(bilevel,true)
}
closed spec fn lzw_variant_witness_bytes(bilevel: bool,legacy:bool) -> Seq<u8> {
    Seq::new(102, |i:int| {
        if i==55 {0u8} else if i==90 {4u8}
        else if i<98 {packbits_witness_bytes(bilevel)[i]}
        else if legacy {seq![0u8,247u8,4u8,4u8][i-98]}
        else {seq![128u8,30u8,224u8,32u8][i-98]}
    })
}

pub closed spec fn lzw_witness_limits() -> Limits {
    Limits { max_input:102, max_dimension:1, max_output:1, max_directories:1, max_tags:7, max_segments:1 }
}

pub proof fn lzw_profile_has_witness(bilevel: bool)
    ensures supported_profile(lzw_witness_bytes(bilevel),0,lzw_witness_limits()),
        inspection_work_bound(0,lzw_witness_limits())==249,
{ lzw_variant_profile_has_witness(bilevel,false); }
pub proof fn lzw_legacy_profile_has_witness(bilevel: bool)
    ensures supported_profile(lzw_legacy_witness_bytes(bilevel),0,lzw_witness_limits()),
        inspection_work_bound(0,lzw_witness_limits())==249,
{ lzw_variant_profile_has_witness(bilevel,true); }

proof fn lzw_variant_profile_has_witness(bilevel: bool,legacy:bool)
    ensures supported_profile(lzw_variant_witness_bytes(bilevel,legacy), 0, lzw_witness_limits()),
        inspection_work_bound(0, lzw_witness_limits()) == 249,
{
    reveal(alpha_field_valid);
    let s = lzw_variant_witness_bytes(bilevel,legacy);
    let limits = lzw_witness_limits();
    let d = Directory { start: 10, count: 7, next: 0 };
    reveal(tag_valid);
    assert forall|i: int| 0 <= i < 7 implies #[trigger] tag_valid(s, 10 + 12 * i, true) by {
        if i == 0 {} else if i == 1 {} else if i == 2 {}
        else if i == 3 {} else if i == 4 {} else if i == 5 {} else { assert(i == 6); }
    }
    assert forall|i: int| 0 <= i < 7 implies
        #[trigger] word16(s, 10 + 12 * i, true) == seq![256int, 257, 258, 259, 262, 273, 279][i] by {
        if i == 0 {} else if i == 1 {} else if i == 2 {}
        else if i == 3 {} else if i == 4 {} else if i == 5 {} else { assert(i == 6); }
    }
    lookup_at(s, d, 256, true, 0); lookup_at(s, d, 257, true, 1);
    lookup_at(s, d, 258, true, 2); lookup_at(s, d, 259, true, 3); lookup_at(s, d, 262, true, 4);
    lookup_at(s, d, 273, true, 5); lookup_at(s, d, 279, true, 6);
    assert forall|id: u16| id != 256 && id != 257 && id != 258 && id != 259 && id != 262 && id != 273 && id != 279
        implies #[trigger] lookup(s, d, id, true).is_none() by {
        assert forall|i: int| 0 <= i < 7 implies #[trigger] word16(s, 10 + 12 * i, true) != id by {
            if i == 0 {} else if i == 1 {} else if i == 2 {}
            else if i == 3 {} else if i == 4 {} else if i == 5 {} else { assert(i == 6); }
        }
        lookup_missing(s, d, id, true);
    }
    reveal(profile_fields);
    reveal(profile_layout);
    reveal(segment_valid);
    assert(directory_valid(s, 8, true, limits));
    assert(bits_model(s, d, true) == if bilevel { 1int } else { 8int });
    let bit_tag = Tag { kind: 3, count: 1, data: 42 };
    assert(lookup(s, d, 258, true) == Some(bit_tag));
    assert forall|i: int| 0 <= i < 1 implies value_valid(s, bit_tag, i)
        && #[trigger] tag_value(s, bit_tag, i, true) == (if bilevel { 1int } else { 8int }) by {
        assert(i == 0);
    }
    assert(bits_supported(s, d, 1, true));
    assert(lookup(s, d, 277, true).is_none());
    assert(lookup(s, d, 266, true).is_none());
    assert(lookup(s, d, 274, true).is_none());
    assert(lookup(s, d, 284, true).is_none());
    assert(lookup(s, d, 317, true).is_none());
    assert(lookup(s, d, 338, true).is_none());
    assert(lookup(s, d, 339, true).is_none());
    assert(scalar_model(s, d, 256, 0, true) == 1);
    assert(scalar_model(s, d, 257, 0, true) == 1);
    assert(scalar_model(s, d, 277, 1, true) == 1);
    assert(scalar_model(s, d, 262, 999, true) == 1);
    assert(scalar_valid(s, d, 256, true) && scalar_valid(s, d, 257, true));
    assert(scalar_valid(s, d, 277, true) && scalar_valid(s, d, 262, true));
    assert(scalar_valid(s, d, 259, true) && scalar_model(s, d, 259, 1, true) == 5);
    assert(scalar_valid(s, d, 274, true) && scalar_model(s, d, 274, 1, true) == 1);
    assert(scalar_valid(s, d, 284, true) && scalar_model(s, d, 284, 1, true) == 1);
    assert(scalar_valid(s, d, 266, true) && scalar_model(s, d, 266, 1, true) == 1);
    assert(scalar_valid(s, d, 317, true) && scalar_model(s, d, 317, 1, true) == 1);
    assert(lookup(s, d, 338, true).is_none() && lookup(s, d, 339, true).is_none());
    assert(profile_fields(s, d, true, limits));
    assert(lookup(s, d, 324, true).is_none());
    assert(lookup(s, d, 278, true).is_none());
    assert(lookup(s, d, 277, true).is_none());
    let g = profile_geometry(s, d, true);
    assert(g.width == 1 && g.height == 1 && g.channels == 1);
    assert(g.block_width == 1 && g.block_height == 4294967295 && g.across == 1);
    assert(g.offsets == (Tag { kind: 4, count: 1, data: 78 }));
    assert(g.counts == (Tag { kind: 4, count: 1, data: 90 }));
    assert(g.le && !g.white);
    assert(g == (Geometry { width: 1, height: 1, channels: 1, alpha:0,
        block_width: 1, block_height: 4294967295, across: 1,
        offsets: Tag { kind: 4, count: 1, data: 78 },
        counts: Tag { kind: 4, count: 1, data: 90 }, le: true, white: false, bilevel, lsb: false, packbits: false, lzw: true, tiled: false }));
    assert(segment_count(g) == 1);
    reveal(lzw_valid);reveal(lz_legacy);
    if legacy {
        assert(lz_lsb_bits(s,784,9)==256) by(compute);
        assert(lz_lsb_bits(s,793,9)==123) by(compute);
        assert(lz_lsb_bits(s,802,9)==257) by(compute);
    } else {
        assert(lz_bits(s,784,9)==256) by(compute);
        assert(lz_bits(s,793,9)==123) by(compute);
        assert(lz_bits(s,802,9)==257) by(compute);
    }
    reveal_with_fuel(lz_valid,3);
    assert(lzw_valid(s,98,102,1));
    assert(segment_valid(s, g, false, 0));
    assert forall|j: int| 0 <= j < segment_count(g) implies segment_valid(s, g, false, j) by {
        assert(j == 0);
    }
    reveal_with_fuel(codec_work_prefix,2);
    assert(codec_work_prefix(s,g,1) == 5);
    assert(profile_layout(s, d, true, limits));
    reveal(supported_profile);
}

fn decode_pb_segment(s: &[u8], g: Geometry, j: usize, output: &mut [u8]) -> (r: Result<(), Error>)
    requires g.packbits, g.positive(), segment_valid(s@,g,g.tiled,j as int),
        g.across as int == (g.width as int-1)/(g.block_width as int)+1,
        g.width as int*g.height*g.channels <= old(output).len(),
        forall|k: int| 0 <= k < g.width as int*g.height*g.channels ==> sample_valid(s@,g,k),
    ensures r.is_ok(), final(output).len() == old(output).len(),
        forall|k: int| 0 <= k < g.width as int*g.height*g.channels ==>
            final(output)@[k] == (if g.segment(k) == j { storage_pixel(s@,g,k) } else { old(output)@[k] }),
        forall|k: int| g.width as int*g.height*g.channels <= k < old(output).len() ==>
            final(output)@[k] == old(output)@[k],
{
    reveal(segment_valid);
    let start=value(s,g.offsets,j,g.le)?;
    let count=value(s,g.counts,j,g.le)?;
    let stride=if g.bilevel { (g.block_width-1)/8+1 } else { mul(g.block_width,g.channels)? };
    let rows=if g.tiled { g.block_height } else {
        let first=mul(j,g.block_height)?;
        if g.height-first < g.block_height { g.height-first } else { g.block_height }
    };
    assert(stride as int*rows <= usize::MAX) by(nonlinear_arith)
        requires g.bilevel ==> stride as int*rows <= usize::MAX,
            !g.bilevel ==> g.block_width as int*rows*g.channels <= usize::MAX,
            !g.bilevel ==> stride as int == g.block_width as int*g.channels;
    let needed=mul(stride,rows)?;
    assert(needed as int == storage_bytes(g,g.tiled,j as int)) by(nonlinear_arith)
        requires needed as int == stride as int*rows,
            rows as int == stored_rows(g,g.tiled,j as int),
            g.bilevel ==> stride as int == g.packed_row_bytes(),
            !g.bilevel ==> stride as int == g.block_width as int*g.channels;
    let mut cursor=PbCursor::new(s,start,count,stride,needed);
    let mut i=0usize;
    while i<needed
        invariant i<=needed, cursor.valid(s@), cursor.emitted == i,
            g.positive(), g.packbits,
            g.across as int == (g.width as int-1)/(g.block_width as int)+1,
            cursor._start == start, cursor.end == start+count, cursor._needed == needed,
            cursor._stride == stride, stride as int == g.row_bytes(),
            needed as int == storage_bytes(g,g.tiled,j as int),
            start as int == tag_value(s@,g.offsets,j as int,g.le),
            count as int == tag_value(s@,g.counts,j as int,g.le),
            output.len() == old(output).len(),
            g.width as int*g.height*g.channels <= output.len(),
            forall|k: int| 0 <= k < g.width as int*g.height*g.channels ==> sample_valid(s@,g,k),
            forall|k: int| 0 <= k < g.width as int*g.height*g.channels ==>
                #[trigger] output@[k] == (if g.segment(k) == j && g.local(k) < i {
                    storage_pixel(s@,g,k)
                } else { old(output)@[k] }),
            forall|k: int| g.width as int*g.height*g.channels <= k < old(output).len() ==>
                output@[k] == old(output)@[k],
        decreases needed-i,
    {
        let byte=cursor.next(s);
        let ghost prior_output=output@;
        let ghost prior_i=i;
        proof {
            assert forall|k: int| 0 <= k < g.width as int*g.height*g.channels
                && g.segment(k) == j && g.local(k) == i
                implies storage_pixel(s@,g,k) == normalized(byte,g,k) by {}
        }
        scatter_byte(g,j,i,stride,byte,output);
        i=i+1;
        proof {
            assert forall|k: int| 0 <= k < g.width as int*g.height*g.channels implies
                #[trigger] output@[k] == (if g.segment(k) == j && g.local(k) < i {
                    storage_pixel(s@,g,k)
                } else { old(output)@[k] }) by {
                assert(prior_output[k] == (if g.segment(k) == j && g.local(k) < prior_i {
                    storage_pixel(s@,g,k)
                } else { old(output)@[k] }));
                if g.segment(k) == j && g.local(k) == prior_i {
                    assert(storage_pixel(s@,g,k) == normalized(byte,g,k));
                }
            }
        }
    }
    proof {
        assert forall|k: int| 0 <= k < g.width as int*g.height*g.channels && g.segment(k) == j
            implies 0 <= #[trigger] g.local(k) < needed by {
            assert(sample_valid(s@,g,k));
        }
    }
    Ok(())
}

fn decode_lz_segment(s: &[u8], g: Geometry, j: usize, output: &mut [u8]) -> (r: Result<(), Error>)
    requires g.lzw, g.positive(), segment_valid(s@,g,g.tiled,j as int),
        g.across as int == (g.width as int-1)/(g.block_width as int)+1,
        g.width as int*g.height*g.channels <= old(output).len(),
        forall|k: int| 0 <= k < g.width as int*g.height*g.channels ==> sample_valid(s@,g,k),
    ensures r.is_ok(), final(output).len() == old(output).len(),
        forall|k: int| 0 <= k < g.width as int*g.height*g.channels ==>
            final(output)@[k] == (if g.segment(k) == j { storage_pixel(s@,g,k) } else { old(output)@[k] }),
        forall|k: int| g.width as int*g.height*g.channels <= k < old(output).len() ==>
            final(output)@[k] == old(output)@[k],
{
    reveal(segment_valid);
    let start=value(s,g.offsets,j,g.le)?;
    let count=value(s,g.counts,j,g.le)?;
    let stride=if g.bilevel { (g.block_width-1)/8+1 } else { mul(g.block_width,g.channels)? };
    let rows=if g.tiled { g.block_height } else {
        let first=mul(j,g.block_height)?;
        if g.height-first < g.block_height { g.height-first } else { g.block_height }
    };
    assert(stride as int*rows <= usize::MAX) by(nonlinear_arith)
        requires g.bilevel ==> stride as int*rows <= usize::MAX,
            !g.bilevel ==> g.block_width as int*rows*g.channels <= usize::MAX,
            !g.bilevel ==> stride as int == g.block_width as int*g.channels;
    let needed=mul(stride,rows)?;
    assert(needed as int == storage_bytes(g,g.tiled,j as int)) by(nonlinear_arith)
        requires needed as int == stride as int*rows,
            rows as int == stored_rows(g,g.tiled,j as int),
            g.bilevel ==> stride as int == g.packed_row_bytes(),
            !g.bilevel ==> stride as int == g.block_width as int*g.channels;
    let mut cursor=LzCursor::new(s,start,count,needed);
    let mut i=0usize;
    while i<needed
        invariant i<=needed, cursor.valid(s@), cursor.emitted == i,
            g.positive(), g.lzw,
            g.across as int == (g.width as int-1)/(g.block_width as int)+1,
            cursor._start == start, cursor.bits.end == start+count, cursor._needed == needed,
            stride as int == g.row_bytes(),
            needed as int == storage_bytes(g,g.tiled,j as int),
            start as int == tag_value(s@,g.offsets,j as int,g.le),
            count as int == tag_value(s@,g.counts,j as int,g.le),
            output.len() == old(output).len(),
            g.width as int*g.height*g.channels <= output.len(),
            forall|k: int| 0 <= k < g.width as int*g.height*g.channels ==> sample_valid(s@,g,k),
            forall|k: int| 0 <= k < g.width as int*g.height*g.channels ==>
                #[trigger] output@[k] == (if g.segment(k) == j && g.local(k) < i {
                    storage_pixel(s@,g,k)
                } else { old(output)@[k] }),
            forall|k: int| g.width as int*g.height*g.channels <= k < old(output).len() ==>
                output@[k] == old(output)@[k],
        decreases needed-i,
    {
        let byte=cursor.next(s);
        let ghost prior_output=output@;
        let ghost prior_i=i;
        proof {
            assert forall|k: int| 0 <= k < g.width as int*g.height*g.channels
                && g.segment(k) == j && g.local(k) == i
                implies storage_pixel(s@,g,k) == normalized(byte,g,k) by {}
        }
        scatter_byte(g,j,i,stride,byte,output);
        i=i+1;
        proof {
            assert forall|k: int| 0 <= k < g.width as int*g.height*g.channels implies
                #[trigger] output@[k] == (if g.segment(k) == j && g.local(k) < i {
                    storage_pixel(s@,g,k)
                } else { old(output)@[k] }) by {
                assert(prior_output[k] == (if g.segment(k) == j && g.local(k) < prior_i {
                    storage_pixel(s@,g,k)
                } else { old(output)@[k] }));
                if g.segment(k) == j && g.local(k) == prior_i {
                    assert(storage_pixel(s@,g,k) == normalized(byte,g,k));
                }
            }
        }
    }
    proof {
        assert forall|k: int| 0 <= k < g.width as int*g.height*g.channels && g.segment(k) == j
            implies 0 <= #[trigger] g.local(k) < needed by {
            assert(sample_valid(s@,g,k));
        }
    }
    Ok(())
}

// A complete byte row in an eight-bit strip is contiguous in both buffers.
// This lemma relates that bulk operation to the unchanged per-sample model.
proof fn strip_row_mapping(g:Geometry,y:int,i:int)
    requires g.positive(), !g.bilevel, g.block_width==g.width, g.across==1,
        0<=y<g.height, 0<=i<g.width as int*g.channels,
    ensures g.segment(y*g.width*g.channels+i)==y/(g.block_height as int),
        g.local(y*g.width*g.channels+i)==(y%(g.block_height as int))*g.width*g.channels+i,
{
    let w=g.width as int;let ch=g.channels as int;
    let x=i/ch;let c=i%ch;let p=y*w+x;let k=y*w*ch+i;
    quotient_remainder(i,ch);
    assert(0<=x<w) by(nonlinear_arith)
        requires 0<ch,0<w,0<=i<w*ch,i==ch*x+c,0<=c<ch,0<=x;
    assert(k==p*ch+c) by(nonlinear_arith)
        requires k==y*w*ch+i,p==y*w+x,i==ch*x+c;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k,ch,p,c);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(p,w,y,x);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(x,w,0,x);
    assert(g.local(k)==(y%(g.block_height as int))*w*ch+i) by(nonlinear_arith)
        requires g.local(k)==((y%(g.block_height as int))*w+x)*ch+c,i==ch*x+c;
}

// Any visible run confined to one tile row is contiguous in storage.
proof fn tile_row_mapping(g:Geometry,y:int,x:int,n:int,i:int)
    requires g.positive(),!g.bilevel,0<=y<g.height,0<=x<g.width,
        0<n<=g.width as int-x,x%(g.block_width as int)+n<=g.block_width,
        0<=i<n*g.channels,
    ensures g.segment((y*g.width+x)*g.channels+i)==y/(g.block_height as int)*g.across+x/(g.block_width as int),
        g.local((y*g.width+x)*g.channels+i)==((y%(g.block_height as int))*g.block_width+x%(g.block_width as int))*g.channels+i,
{
    let w=g.width as int;let ch=g.channels as int;let bw=g.block_width as int;
    let ix=i/ch;let c=i%ch;let xx=x+ix;let p=y*w+xx;let k=(y*w+x)*ch+i;
    quotient_remainder(i,ch);quotient_remainder(x,bw);
    assert(0<=ix<n) by(nonlinear_arith)
        requires 0<ch,0<n,0<=i<n*ch,i==ch*ix+c,0<=c<ch,0<=ix;
    assert(k==p*ch+c) by(nonlinear_arith)
        requires k==(y*w+x)*ch+i,p==y*w+x+ix,i==ch*ix+c;
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k,ch,p,c);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(p,w,y,xx);
    assert(xx==bw*(x/bw)+(x%bw+ix));
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(xx,bw,x/bw,x%bw+ix);
    assert(g.local(k)==((y%(g.block_height as int))*bw+x%bw)*ch+i) by(nonlinear_arith)
        requires g.local(k)==((y%(g.block_height as int))*bw+x%bw+ix)*ch+c,i==ch*ix+c;
}

// A full-width strip owns one contiguous span of interleaved output bytes,
// and its local index is the offset inside that span.
proof fn strip_span(g:Geometry,k:int,rows:int,q:int)
    requires g.positive(),!g.bilevel,g.block_width==g.width,g.across==1,0<=k,1<=rows<=g.block_height,
        k*g.block_height+rows<=g.height,rows==g.block_height || k*g.block_height+rows==g.height,
        0<=q<g.width as int*g.height*g.channels,
    ensures (g.segment(q)==k)==(k*g.block_height*g.width*g.channels<=q<(k*g.block_height+rows)*g.width*g.channels),
        g.segment(q)==k ==> g.local(q)==q-k*g.block_height*g.width*g.channels,
{
    let ch=g.channels as int;let w=g.width as int;let bh=g.block_height as int;let h=g.height as int;
    let p=q/ch;let c=q%ch;quotient_remainder(q,ch);
    let y=p/w;let x=p%w;quotient_remainder(p,w);
    let s=y/bh;let r=y%bh;quotient_remainder(y,bh);
    vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(x,w,0,x);
    assert(g.segment(q)==s);
    assert(g.local(q)==(r*w+x)*ch+c);
    let base=k*bh*w*ch;
    assert(0<=y<h) by(nonlinear_arith) requires 0<=p,0<=x<w,p==w*y+x,0<=q<w*h*ch,q==ch*p+(q%ch),0<=q%ch<ch;
    if s==k {
        assert(r<rows) by(nonlinear_arith) requires y==bh*s+r,s==k,y<h,r<bh,rows==bh || k*bh+rows==h;
        assert(q-base==(r*w+x)*ch+c) by(nonlinear_arith) requires q==ch*p+c,p==w*y+x,y==bh*s+r,s==k,base==k*bh*w*ch;
        assert(base<=q<(k*bh+rows)*w*ch) by(nonlinear_arith)
            requires q-base==(r*w+x)*ch+c,0<=r<rows,0<=x<w,0<=c<ch,base==k*bh*w*ch,w>0,ch>0;
    } else if s<k {
        assert(y<k*bh) by(nonlinear_arith) requires y==bh*s+r,r<bh,s<k,bh>0;
        assert(q<base) by(nonlinear_arith) requires q==ch*p+c,p==w*y+x,y<k*bh,0<=x<w,0<=c<ch,base==k*bh*w*ch,w>0,ch>0;
    } else {
        assert(y>=k*bh+rows) by(nonlinear_arith) requires y==bh*s+r,0<=r,s>k,rows<=bh,bh>0;
        assert(q>=(k*bh+rows)*w*ch) by(nonlinear_arith) requires q==ch*p+c,p==w*y+x,y>=k*bh+rows,0<=x,0<=c,w>0,ch>0;
    }
}

fn copy_byte_run(input:&[u8],src:usize,output:&mut [u8],dst:usize,n:usize,invert:bool)
    requires src as int+n<=input.len(),dst as int+n<=old(output).len(),
    ensures final(output).len()==old(output).len(),
        forall|i:int| 0<=i<n ==> #[trigger] final(output)@[dst as int+i]==
            (if invert {(255-input@[src as int+i]) as u8} else {input@[src as int+i]}),
        forall|i:int| 0<=i<old(output).len() && !(dst<=i<dst as int+n) ==> final(output)@[i]==old(output)@[i],
{
    let (_,source_tail)=input.split_at(src);
    let (source,_)=source_tail.split_at(n);
    let (_,target_tail)=output.split_at_mut(dst);
    let (target,_)=target_tail.split_at_mut(n);
    if !invert {
        target.copy_from_slice(source);
    } else {
        let mut i=0usize;
        while i<n
            invariant i<=n,source.len()==n,target.len()==n,
                forall|j:int| 0<=j<i ==> target@[j]==(255-source@[j]) as u8,
            decreases n-i,
        {
            target[i]=255-source[i];i=i+1;
        }
    }
}

pub struct Image<'a> {
    input: &'a [u8],
    geometry: Geometry,
    output_len: usize,
    decode_work: usize,
    directory: Directory,
    page: usize,
}

impl<'a> Image<'a> {
    pub closed spec fn selected_page(&self) -> usize { self.page }
    pub closed spec fn metadata_matches_source(&self) -> bool {
        let s = self.input@;
        let g = self.geometry;
        let d = self.directory;
        let tiled = lookup(s, d, 324, g.le).is_some();
        &&& d.start == nth_ifd(s, word32(s, 4, g.le), g.le, self.page as nat) + 2
        &&& d.count == word16(s, d.start as int - 2, g.le)
        &&& word16(s, 2, g.le) == 42
        &&& (if g.le { s[0] == 73 && s[1] == 73 } else { s[0] == 77 && s[1] == 77 })
        &&& g.width == scalar_model(s, d, 256, 0, g.le)
        &&& g.height == scalar_model(s, d, 257, 0, g.le)
        &&& g.channels == scalar_model(s, d, 277, 1, g.le)
        &&& (1 <= g.channels <= 4)
        &&& (if g.alpha==0 {g.channels==1 || g.channels==3} else {g.channels==2 || g.channels==4})
        &&& (if g.channels <= 2 {
            scalar_model(s, d, 262, 999, g.le) == 0 || scalar_model(s, d, 262, 999, g.le) == 1
        } else { scalar_model(s, d, 262, 999, g.le) == 2 })
        &&& g.white == (scalar_model(s, d, 262, 999, g.le) == 0)
        &&& (scalar_model(s, d, 259, 1, g.le) == 1 || scalar_model(s, d, 259, 1, g.le) == 32773 || scalar_model(s, d, 259, 1, g.le) == 5)
        &&& g.packbits == (scalar_model(s, d, 259, 1, g.le) == 32773)
        &&& g.lzw == (scalar_model(s, d, 259, 1, g.le) == 5)
        &&& g.tiled == tiled
        &&& ((g.packbits || g.lzw) ==> !g.lsb)
        &&& scalar_model(s, d, 274, 1, g.le) == 1
        &&& scalar_model(s, d, 284, 1, g.le) == 1
        &&& (scalar_model(s, d, 266, 1, g.le) == 1
            || (g.bilevel && scalar_model(s, d, 266, 1, g.le) == 2))
        &&& g.lsb == (scalar_model(s, d, 266, 1, g.le) == 2)
        &&& scalar_model(s, d, 317, 1, g.le) == 1
        &&& alpha_field_valid(s,d,g.le)
        &&& g.alpha == scalar_model(s,d,338,0,g.le)
        &&& bits_supported(s, d, g.channels as int, g.le)
        &&& g.bilevel == (bits_model(s, d, g.le) == 1)
        &&& (match lookup(s, d, 339, g.le) {
            None => true,
            Some(t) => t.kind == 3 && t.count == g.channels
                && (forall|i: int| 0 <= i < g.channels ==> #[trigger] tag_value(s, t, i, g.le) == 1),
        })
        &&& g.block_width == (if tiled { scalar_model(s, d, 322, 0, g.le) } else { g.width as int })
        &&& g.block_height == (if tiled { scalar_model(s, d, 323, 0, g.le) } else { scalar_model(s, d, 278, 4294967295, g.le) })
        &&& g.across == ((g.width as int - 1) / (g.block_width as int)) + 1
        &&& lookup(s, d, if tiled { 324 } else { 273 }, g.le) == Some(g.offsets)
        &&& lookup(s, d, if tiled { 325 } else { 279 }, g.le) == Some(g.counts)
    }
    pub closed spec fn byte_len(&self) -> usize { self.output_len }
    pub closed spec fn work_len(&self) -> usize { self.decode_work }
    pub fn decode_work_limit(&self) -> (r: usize)
        ensures r == self.work_len(),
    { self.decode_work }
    pub closed spec fn source_compression(&self) -> int {
        scalar_model(self.input@,self.directory,259,1,self.geometry.le)
    }
    pub fn compression(&self) -> (r: usize)
        ensures r == 1 || r == 32773 || r == 5,
            self.well_formed() ==> r == self.source_compression(),
    { if self.geometry.packbits {32773} else if self.geometry.lzw {5} else {1} }
    pub closed spec fn source_bytes(&self) -> Seq<u8> { self.input@ }
    pub closed spec fn dimensions(&self) -> (usize, usize, usize) {
        (self.geometry.width, self.geometry.height, self.geometry.channels)
    }
    pub closed spec fn tag_count(&self) -> usize { self.directory.count }
    pub fn page_index(&self) -> (r: usize)
        ensures r == self.selected_page(),
    { self.page }
    pub fn directory_entry_count(&self) -> (r: usize)
        ensures r == self.tag_count(),
    { self.directory.count }
    pub fn width(&self) -> (r: usize)
        ensures r == self.dimensions().0,
    { self.geometry.width }
    pub fn height(&self) -> (r: usize)
        ensures r == self.dimensions().1,
    { self.geometry.height }
    pub fn channels(&self) -> (r: usize)
        ensures r == self.dimensions().2,
    { self.geometry.channels }
    /// Encoding of the source alpha; decode_into emits straight-alpha pixels.
    pub closed spec fn source_alpha(&self) -> int {scalar_model(self.input@,self.directory,338,0,self.geometry.le)}
    pub fn alpha_mode(&self) -> (r:AlphaMode)
        ensures self.well_formed() ==> r == (if self.source_alpha()==1 {AlphaMode::Associated}
            else if self.source_alpha()==2 {AlphaMode::Unassociated} else {AlphaMode::None}),
    { if self.geometry.alpha==1 {AlphaMode::Associated} else if self.geometry.alpha==2 {AlphaMode::Unassociated} else {AlphaMode::None} }
    pub fn has_alpha(&self) -> (r:bool)
        ensures self.well_formed() ==> r==(self.source_alpha()!=0),
    {self.geometry.alpha!=0}
    /// Source sample depth; decode_into always emits 8-bit output samples.
    pub closed spec fn source_sample_depth(&self) -> int {
        bits_model(self.input@, self.directory, self.geometry.le)
    }
    pub fn bits_per_sample(&self) -> (r: usize)
        ensures r == 1 || r == 8,
            self.well_formed() ==> r == self.source_sample_depth(),
    { if self.geometry.bilevel { 1 } else { 8 } }
    pub fn output_len(&self) -> (r: usize)
        ensures r == self.byte_len(),
    { self.output_len }

    pub closed spec fn well_formed(&self) -> bool {
        self.geometry.positive()
        && self.metadata_matches_source()
        && self.output_len as int == self.geometry.width as int * self.geometry.height as int * self.geometry.channels as int
        && self.decode_work as int == decode_base(self.geometry) + (if self.geometry.packbits || self.geometry.lzw {
            codec_work_prefix(self.input@,self.geometry,segment_count(self.geometry) as nat)
        } else { 0int })
        && ((self.geometry.packbits || self.geometry.lzw) ==> segment_count(self.geometry) <= usize::MAX
            && self.geometry.offsets.count == segment_count(self.geometry)
            && self.geometry.counts.count == segment_count(self.geometry)
            && forall|j: int| 0 <= j < segment_count(self.geometry) ==>
                segment_valid(self.input@,self.geometry,self.geometry.tiled,j))
        && forall|k: int| 0 <= k < self.output_len ==> sample_valid(self.input@, self.geometry, k)
    }

    pub closed spec fn decoded_byte(&self, k: int) -> u8 {
        output_pixel(self.input@, self.geometry, k)
    }

    fn decode_compressed(&self, output: &mut [u8]) -> (r: Result<(), Error>)
        requires self.well_formed(), (self.geometry.packbits || self.geometry.lzw),
            old(output).len() >= self.byte_len(),
        ensures r.is_ok(), final(output).len() == old(output).len(),
            forall|k: int| 0 <= k < self.byte_len() ==> final(output)@[k] == storage_pixel(self.input@,self.geometry,k),
            forall|k: int| self.byte_len() <= k < old(output).len() ==> final(output)@[k] == old(output)@[k],
    {
        let g=self.geometry;
        let mut j=0usize;
        while j<g.counts.count
            invariant self.well_formed(), g == self.geometry, (g.packbits || g.lzw), j<=g.counts.count,
                output.len() == old(output).len(), output.len() >= self.byte_len(),
                forall|k: int| 0 <= k < self.byte_len() ==> output@[k] ==
                    (if g.segment(k) < j { storage_pixel(self.input@,self.geometry,k) } else { old(output)@[k] }),
                forall|k: int| self.byte_len() <= k < old(output).len() ==> output@[k] == old(output)@[k],
            decreases g.counts.count-j,
        {
            assert(segment_valid(self.input@,g,g.tiled,j as int));
            if g.lzw { decode_lz_segment(self.input,g,j,output)?; }
            else { decode_pb_segment(self.input,g,j,output)?; }
            j=j+1;
        }
        proof {
            assert forall|k: int| 0 <= k < self.byte_len() implies
                0 <= #[trigger] g.segment(k) < g.counts.count by {
                assert(sample_valid(self.input@,g,k));
            }
        }
        Ok(())
    }

    fn decode_uncompressed_storage(&self,output:&mut [u8]) -> (r:Result<(),Error>)
        requires self.well_formed(), !self.geometry.packbits && !self.geometry.lzw,
            old(output).len()>=self.byte_len(),
        ensures r.is_ok(), final(output).len()==old(output).len(),
            forall|i:int| 0<=i<self.byte_len() ==> final(output)@[i]==storage_pixel(self.input@,self.geometry,i),
            forall|i:int| self.byte_len()<=i<old(output).len() ==> final(output)@[i]==old(output)@[i],
    {
        if !self.geometry.bilevel {
            if !self.geometry.tiled {return self.decode_uncompressed_rows(output);}
            return self.decode_uncompressed_tile_rows(output);
        }
        let mut k=0usize;
        while k<self.output_len
            invariant self.well_formed(), !self.geometry.packbits && !self.geometry.lzw,
                k<=self.output_len<=output.len(), output.len()==old(output).len(),
                forall|i:int| 0<=i<k ==> output@[i]==storage_pixel(self.input@,self.geometry,i),
                forall|i:int| self.output_len<=i<output.len() ==> output@[i]==old(output)@[i],
            decreases self.output_len-k,
        {
            output[k]=sample(self.input,self.geometry,k)?;
            k=k+1;
        }
        Ok(())
    }

    fn decode_uncompressed_tile_rows(&self,output:&mut [u8]) -> (r:Result<(),Error>)
        requires self.well_formed(),!self.geometry.packbits && !self.geometry.lzw,
            !self.geometry.bilevel,old(output).len()>=self.byte_len(),
        ensures r.is_ok(),final(output).len()==old(output).len(),
            forall|i:int| 0<=i<self.byte_len() ==> final(output)@[i]==storage_pixel(self.input@,self.geometry,i),
            forall|i:int| self.byte_len()<=i<old(output).len() ==> final(output)@[i]==old(output)@[i],
    {
        let g=self.geometry;let mut k=0usize;
        while k<self.output_len
            invariant self.well_formed(),g==self.geometry,!g.bilevel,!g.packbits,!g.lzw,
                k<=self.output_len,k%g.channels==0,
                output.len()==old(output).len(),output.len()>=self.output_len,
                forall|i:int| 0<=i<k ==> output@[i]==storage_pixel(self.input@,g,i),
                forall|i:int| self.output_len<=i<output.len() ==> output@[i]==old(output)@[i],
            decreases self.output_len-k,
        {
            let p=k/g.channels;let x=p%g.width;let y=p/g.width;
            proof {
                quotient_remainder(k as int,g.channels as int);
                quotient_remainder(p as int,g.width as int);
                assert(p as int==y as int*g.width+x) by(nonlinear_arith)
                    requires p as int==g.width as int*y+x;
                assert(p<g.width as int*g.height) by(nonlinear_arith)
                    requires k<self.output_len,self.output_len as int==g.width as int*g.height*g.channels,
                        k as int==p as int*g.channels,g.channels>0;
                assert(y<g.height) by(nonlinear_arith)
                    requires p<g.width as int*g.height,p as int==y as int*g.width+x,x>=0,g.width>0;
            }
            let tile_left=g.block_width-x%g.block_width;
            let image_left=g.width-x;
            let pixels=if tile_left<image_left {tile_left} else {image_left};
            proof {
                assert(pixels>0);
                assert(pixels as int*g.channels>0) by(nonlinear_arith) requires pixels>0,g.channels>0;
                assert(k as int+pixels as int*g.channels<=self.output_len) by(nonlinear_arith)
                    requires k as int==p as int*g.channels,p as int==y as int*g.width+x,
                        pixels<=g.width-x,y<g.height,g.channels>0,
                        self.output_len as int==g.width as int*g.height*g.channels;
                tile_row_mapping(g,y as int,x as int,pixels as int,0);
                tile_row_mapping(g,y as int,x as int,pixels as int,pixels as int*g.channels-1);
                assert(sample_valid(self.input@,g,k as int));
                assert(sample_valid(self.input@,g,k as int+pixels as int*g.channels-1));
            }
            let count=mul(pixels,g.channels)?;
            let block=add(mul(y/g.block_height,g.across)?,x/g.block_width)?;
            let off=value(self.input,g.offsets,block,g.le)?;
            let local=mul(add(mul(y%g.block_height,g.block_width)?,x%g.block_width)?,g.channels)?;
            let src=add(off,local)?;
            assert(src as int+count<=self.input.len());
            let ghost before=output@;let ghost old_k=k;
            copy_byte_run(self.input,src,output,k,count,g.white && g.alpha==0);
            k=k+count;
            proof {
                assert(k as int==(p as int+pixels)*g.channels) by(nonlinear_arith)
                    requires k as int==old_k as int+count,old_k as int==p as int*g.channels,count as int==pixels as int*g.channels;
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(k as int,g.channels as int,p as int+pixels,0);
                assert forall|i:int| 0<=i<k implies output@[i]==storage_pixel(self.input@,g,i) by {
                    if i>=old_k {
                        tile_row_mapping(g,y as int,x as int,pixels as int,i-old_k);
                        assert(sample_address(self.input@,g,i)==src as int+i-old_k);
                    }
                }
            }
        }
        Ok(())
    }

    fn decode_uncompressed_rows(&self,output:&mut [u8]) -> (r:Result<(),Error>)
        requires self.well_formed(), !self.geometry.packbits && !self.geometry.lzw,
            !self.geometry.bilevel,!self.geometry.tiled,old(output).len()>=self.byte_len(),
        ensures r.is_ok(), final(output).len()==old(output).len(),
            forall|i:int| 0<=i<self.byte_len() ==> final(output)@[i]==storage_pixel(self.input@,self.geometry,i),
            forall|i:int| self.byte_len()<=i<old(output).len() ==> final(output)@[i]==old(output)@[i],
    {
        let g=self.geometry;
        proof {
            vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse(g.width as int-1,g.width as int,0,g.width as int-1);
            assert(g.across==1);
            assert(0<g.width as int*g.channels<=self.output_len) by(nonlinear_arith)
                requires g.width>0,g.channels>0,g.height>0,
                    self.output_len as int==g.width as int*g.height*g.channels;
        }
        let row=mul(g.width,g.channels)?;
        assert(self.output_len as int==g.height as int*row) by(nonlinear_arith)
            requires row as int==g.width as int*g.channels,
                self.output_len as int==g.width as int*g.height*g.channels;
        let mut y=0usize;
        while y<g.height
            invariant self.well_formed(),g==self.geometry,!g.bilevel,!g.tiled,!g.packbits,!g.lzw,
                g.block_width==g.width,g.across==1,row as int==g.width as int*g.channels,row>0,
                y<=g.height,self.output_len as int==g.height as int*row,y as int*row<=self.output_len,
                output.len()==old(output).len(),output.len()>=self.output_len,
                forall|i:int| 0<=i<y as int*row ==> output@[i]==storage_pixel(self.input@,g,i),
                forall|i:int| self.output_len<=i<output.len() ==> output@[i]==old(output)@[i],
            decreases g.height-y,
        {
            proof {
                assert(0<=y as int*row && y as int*row+row<=self.output_len) by(nonlinear_arith)
                    requires 0<=y<g.height,row>0,self.output_len as int==g.height as int*row;
                assert(y as int*row==y as int*g.width*g.channels) by(nonlinear_arith)
                    requires row as int==g.width as int*g.channels;
                strip_row_mapping(g,y as int,0);
                strip_row_mapping(g,y as int,row as int-1);
                assert(sample_valid(self.input@,g,y as int*row));
                assert(sample_valid(self.input@,g,y as int*row+row-1));
                assert(g.segment(y as int*row)==y as int/(g.block_height as int));
                assert(value_valid(self.input@,g.offsets,y as int/(g.block_height as int)));
                assert((y as int%(g.block_height as int))*row==g.local(y as int*row)) by(nonlinear_arith)
                    requires row as int==g.width as int*g.channels,
                        g.local(y as int*row)==(y as int%(g.block_height as int))*g.width*g.channels;
            }
            let dst=mul(y,row)?;
            let off=value(self.input,g.offsets,y/g.block_height,g.le)?;
            let local=mul(y%g.block_height,row)?;
            let src=add(off,local)?;
            assert(src as int+row<=self.input.len());
            let ghost before=output@;
            copy_byte_run(self.input,src,output,dst,row,g.white && g.alpha==0);
            proof {
                assert((y as int+1)*row==dst as int+row) by(nonlinear_arith)
                    requires dst as int==y as int*row;
                assert forall|i:int| 0<=i<(y as int+1)*row implies output@[i]==storage_pixel(self.input@,g,i) by {
                    if i>=dst {
                        assert(0<=i-dst<row);
                        strip_row_mapping(g,y as int,i-dst);
                        assert(sample_address(self.input@,g,i)==src as int+i-dst);
                    }
                }
            }
            y=y+1;
        }
        Ok(())
    }

    pub fn decode_into(&self, output: &mut [u8], budget: &mut Budget) -> (r: Result<(), Error>)
        requires self.well_formed(),
        ensures final(budget).remaining <= old(budget).remaining,
            final(output).len() == old(output).len(),
            r.is_ok() == (old(output).len() >= self.byte_len() && old(budget).remaining >= self.work_len()),
            r.is_ok() ==> final(budget).remaining as int + self.work_len() == old(budget).remaining,
            r.is_ok() ==> forall|i: int| 0 <= i < self.byte_len() ==>
                final(output)@[i] == self.decoded_byte(i),
            forall|i: int| self.byte_len() <= i < old(output).len() ==>
                final(output)@[i] == old(output)@[i],
    {
        if output.len() < self.output_len { return Err(Error::OutputTooSmall); }
        // Preserve the original partial-prefix behavior when a plain
        // uncompressed decode runs out of quota. The sufficient-budget path
        // reserves once and uses bulk rows.
        if !self.geometry.packbits && !self.geometry.lzw && self.geometry.alpha==0
            && budget.remaining<self.decode_work {
        let mut k: usize = 0;
        while k < self.output_len
            invariant self.well_formed(), !(self.geometry.packbits || self.geometry.lzw), self.geometry.alpha==0, k <= self.output_len,
                output.len() == old(output).len(), output.len() >= self.output_len,
                budget.remaining as int + k == old(budget).remaining,
                forall|i: int| 0 <= i < k ==> output@[i] == self.decoded_byte(i),
                forall|i: int| self.output_len <= i < output.len() ==> output@[i] == old(output)@[i],
            decreases self.output_len - k,
        {
            spend(budget, 1)?;
            let v = sample(self.input, self.geometry, k)?;
            output[k] = v;
            k = k + 1;
        }
            return Ok(());
        }
        spend(budget,self.decode_work)?;
        if self.geometry.packbits || self.geometry.lzw {self.decode_compressed(output)?;}
        else {self.decode_uncompressed_storage(output)?;}
        if self.geometry.alpha!=0 {convert_alpha(self.input,self.geometry,self.output_len,output);}
        Ok(())
    }
}

// Selected-image scalar and channel metadata is verified independently of
// directory traversal and segment validation, keeping their proof contexts small.
#[derive(Clone,Copy)]
struct Format {
    width:usize,height:usize,channels:usize,photo:usize,compression:usize,
    alpha:usize,depth:usize,fill:usize,output_len:usize,
}
impl Format {
    spec fn matches(self,s:Seq<u8>,d:Directory,le:bool,limits:Limits) -> bool {
        &&& profile_fields(s,d,le,limits)
        &&& self.width==scalar_model(s,d,256,0,le)
        &&& self.height==scalar_model(s,d,257,0,le)
        &&& self.channels==scalar_model(s,d,277,1,le)
        &&& self.photo==scalar_model(s,d,262,999,le)
        &&& self.compression==scalar_model(s,d,259,1,le)
        &&& self.alpha==scalar_model(s,d,338,0,le)
        &&& self.alpha<=2
        &&& self.depth==bits_model(s,d,le)
        &&& self.fill==scalar_model(s,d,266,1,le)
        &&& self.output_len as int==self.width as int*self.height*self.channels
    }
}
fn read_format(s:&[u8],d:Directory,le:bool,limits:&Limits,budget:&mut Budget) -> (r:Result<Format,Error>)
    requires d.readable(s@,le),
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=12*(d.count as int)+4,
        r.is_ok() ==> r.unwrap().matches(s@,d,le,*limits),
        profile_fields(s@,d,le,*limits) && old(budget).remaining as int>=12*(d.count as int)+4 ==> r.is_ok(),
{
    let ghost eligible=profile_fields(s@,d,le,*limits) && budget.remaining as int>=12*(d.count as int)+4;
    reveal(profile_fields);
    let w = scalar(s, d, 256, 0, le, budget)?;
    let height = scalar(s, d, 257, 0, le, budget)?;
    if w == 0 || height == 0 { return Err(Error::Invalid); }
    if w > limits.max_dimension || height > limits.max_dimension { return Err(Error::Limit); }
    let channels = scalar(s, d, 277, 1, le, budget)?;
    if channels < 1 || channels > 4 { return Err(Error::Unsupported); }
    let photo = scalar(s, d, 262, 999, le, budget)?;
    if !((channels <= 2 && (photo == 0 || photo == 1)) || (channels >= 3 && photo == 2)) {
        return Err(Error::Unsupported);
    }
    let compression = scalar(s, d, 259, 1, le, budget)?;
    if compression != 1 && compression != 32773 && compression != 5 { return Err(Error::Unsupported); }
    let packbits = compression == 32773;
    let lzw = compression == 5;
    if scalar(s, d, 274, 1, le, budget)? != 1
        || scalar(s, d, 284, 1, le, budget)? != 1
        || scalar(s, d, 317, 1, le, budget)? != 1 {
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
    if depth != 8 && !(depth == 1 && channels == 1) { return Err(Error::Unsupported); }
    let fill = scalar(s, d, 266, 1, le, budget)?;
    if fill != 1 && !(depth == 1 && fill == 2) { return Err(Error::Unsupported); }
    if (packbits || lzw) && fill != 1 { return Err(Error::Unsupported); }
    let formats = find(s, d, 339, le, budget)?;
    if let Some(t) = formats {
        if t.count != channels || t.kind != 3 { return Err(Error::Invalid); }
    }
    assert(eligible ==> budget.remaining as int >=
        4);
    let ghost before_channels = budget.remaining;
    let mut c: usize = 0;
    while c < channels
        invariant c <= channels, budget.remaining <= old(budget).remaining,
            eligible == (profile_fields(s@,d,le,*limits) && old(budget).remaining as int>=12*(d.count as int)+4),
            eligible ==> before_channels as int >= 4,
            1 <= channels <= 4,
            (depth == 1 && channels == 1) || depth == 8,
            depth == bits_model(s@, d, le),
            bits == lookup(s@, d, 258, le),
            match bits { None => depth == 1, Some(t) => t.count == channels && t.kind == 3 },
            eligible ==> match bits { None => true, Some(t) => array_of(s@, t, channels as int, depth as int, le) },
            eligible ==> match formats { None => true, Some(t) => array_of(s@, t, channels as int, 1, le) },
            budget.remaining as int + c == before_channels,
            old(budget).remaining as int-before_channels<=12*(d.count as int),
            match bits { None => true, Some(t) => forall|i: int| 0 <= i < c ==>
                #[trigger] value_valid(s@, t, i) },
            match bits { None => true, Some(t) => forall|i: int| 0 <= i < c ==>
                #[trigger] tag_value(s@, t, i, le) == depth },
            match formats { None => true, Some(t) => forall|i: int| 0 <= i < c ==> #[trigger] value_valid(s@,t,i) },
            match formats { None => true, Some(t) => forall|i: int| 0 <= i < c ==> #[trigger] tag_value(s@, t, i, le) == 1 },
        decreases channels - c,
    {
        spend(budget, 1)?;
        if let Some(t) = bits {
            proof { if eligible { array_entry(s@, t, channels as int, depth as int, le, c as int); } }
            if value(s, t, c, le)? != depth { return Err(Error::Unsupported); }
        }
        if let Some(t) = formats {
            proof { if eligible { array_entry(s@, t, channels as int, 1, le, c as int); } }
            assert(eligible ==> value_valid(s@, t, c as int)
                && tag_value(s@, t, c as int, le) == 1);
            if value(s, t, c, le)? != 1 { return Err(Error::Unsupported); }
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
        assert(bits_supported(s@, d, channels as int, le));
    }
    let output_len = mul(mul(w, height)?, channels)?;
    if output_len > limits.max_output { return Err(Error::Limit); }
    proof {
        if let Some(t)=formats {
            assert(t.kind==3 && t.count==channels);
            assert forall|i:int| 0<=i<channels implies value_valid(s@,t,i)
                && #[trigger] tag_value(s@,t,i,le)==1 by {
                assert(i<c);
                assert(value_valid(s@,t,i));
                assert(tag_value(s@,t,i,le)==1);
            }
            assert(array_of(s@,t,channels as int,1,le));
        }
    }
    Ok(Format {width:w,height,channels,photo,compression,alpha,depth,fill,output_len})
}

// Parse the seven layout fields separately, keeping the complete original
// acceptance proof modular as additional codecs are linked into the crate.
fn read_geometry(s:&[u8],d:Directory,le:bool,format:Format,limits:&Limits,budget:&mut Budget) -> (r:Result<Geometry,Error>)
    requires d.readable(s@,le),format.matches(s@,d,le,*limits),
    ensures final(budget).remaining<=old(budget).remaining,
        old(budget).remaining as int-final(budget).remaining<=7*(d.count as int),
        profile_layout(s@,d,le,*limits) && old(budget).remaining as int>=7*(d.count as int) ==> r.is_ok(),
        r.is_ok() ==> r.unwrap()==profile_geometry(s@,d,le),
        r.is_ok() ==> lookup(s@,d,if r.unwrap().tiled {324} else {273},le)==Some(r.unwrap().offsets),
        r.is_ok() ==> lookup(s@,d,if r.unwrap().tiled {325} else {279},le)==Some(r.unwrap().counts),
        r.is_ok() ==> r.unwrap().block_width==if r.unwrap().tiled {scalar_model(s@,d,322,0,le)} else {format.width as int},
        r.is_ok() ==> r.unwrap().block_height==if r.unwrap().tiled {scalar_model(s@,d,323,0,le)} else {scalar_model(s@,d,278,4294967295,le)},
        r.is_ok() ==> r.unwrap().positive(),
        r.is_ok() ==> r.unwrap().across as int==(r.unwrap().width as int-1)/(r.unwrap().block_width as int)+1,
        r.is_ok() ==> (!r.unwrap().tiled ==> r.unwrap().block_width==r.unwrap().width),
        r.is_ok() ==> segment_count(r.unwrap())<=limits.max_segments,
        r.is_ok() ==> r.unwrap().offsets.count==segment_count(r.unwrap()) && r.unwrap().counts.count==segment_count(r.unwrap()),
{
    reveal(profile_fields);reveal(profile_layout);
    let w=format.width;let height=format.height;let channels=format.channels;
    let photo=format.photo;let alpha=format.alpha;
    let packbits=format.compression==32773;let lzw=format.compression==5;
    let bilevel=format.depth==1;let lsb=format.fill==2;
    let tiled = find(s, d, 324, le, budget)?.is_some();
    let (bw, bh, offsets, counts) = if tiled {
        if find(s, d, 273, le, budget)?.is_some() || find(s, d, 279, le, budget)?.is_some() {
            return Err(Error::Invalid);
        }
        let tw = scalar(s, d, 322, 0, le, budget)?;
        let th = scalar(s, d, 323, 0, le, budget)?;
        if tw == 0 || th == 0 || tw % 16 != 0 || th % 16 != 0 { return Err(Error::Invalid); }
        (tw, th, required_tag(s, d, 324, le, budget)?, required_tag(s, d, 325, le, budget)?)
    } else {
        if find(s, d, 322, le, budget)?.is_some() || find(s, d, 323, le, budget)?.is_some()
            || find(s, d, 325, le, budget)?.is_some() { return Err(Error::Invalid); }
        let rows = scalar(s, d, 278, 4294967295, le, budget)?;
        if rows == 0 { return Err(Error::Invalid); }
        (w, rows, required_tag(s, d, 273, le, budget)?, required_tag(s, d, 279, le, budget)?)
    };
    let across = (w - 1) / bw + 1;
    let down = (height - 1) / bh + 1;
    let segments = mul(across, down)?;
    if segments > limits.max_segments { return Err(Error::Limit); }
    if offsets.count != segments || counts.count != segments { return Err(Error::Invalid); }
    let geometry = Geometry { width: w, height, channels, alpha, block_width: bw,
        block_height: bh, across, offsets, counts, le, white: photo == 0, bilevel, lsb, packbits, lzw, tiled };
    Ok(geometry)
}

// Keep this large acceptance proof in its own solver context. This changes
// proof scheduling only, not the contracts or executable implementation.
#[verifier::spinoff_prover]
pub fn inspect<'a>(s: &'a [u8], page: usize, limits: &Limits,
    budget: &mut Budget) -> (r: Result<Image<'a>, Error>)
    ensures final(budget).remaining <= old(budget).remaining,
        supported_profile(s@, page, *limits)
            && old(budget).remaining as int >= inspection_work_bound(page, *limits) ==> r.is_ok(),
        r.is_ok() ==> r.unwrap().well_formed(),
        r.is_ok() ==> r.unwrap().metadata_matches_source(),
        r.is_ok() ==> r.unwrap().selected_page() == page,
        r.is_ok() ==> r.unwrap().source_bytes() == s@,
        r.is_ok() ==> r.unwrap().byte_len() <= limits.max_output,
{
    let ghost eligible = supported_profile(s@, page, *limits)
        && budget.remaining as int >= inspection_work_bound(page, *limits);
    reveal(supported_profile);
    if s.len() > limits.max_input { return Err(Error::Limit); }
    if page >= limits.max_directories { return Err(Error::Limit); }
    let h = header(s)?;
    let le = h.little_endian;
    let mut off = h.first_ifd;
    let mut p: usize = 0;
    while p < page
        invariant p <= page, budget.remaining <= old(budget).remaining,
            eligible == (supported_profile(s@, page, *limits)
                && old(budget).remaining as int >= inspection_work_bound(page, *limits)),
            s.len() >= 8, s@.len() == s.len(),
            le == (s@[0] == 73), h.first_ifd == word32(s@, 4, le),
            old(budget).remaining as int - budget.remaining <= p as int * (limits.max_tags as int + 1),
            off == nth_ifd(s@, h.first_ifd as int, le, p as nat),
        decreases page - p,
    {
        reveal(supported_profile);
        assert(eligible ==> directory_valid(s@, off as int, le, *limits));
        proof { if eligible {
            traversal_budget(old(budget).remaining as int, budget.remaining as int,
                p as int, page as int, limits.max_tags as int,
                19 * limits.max_tags + 4 + (limits.max_segments as int)*(limits.max_input as int+1) + limits.max_output);
        } }
        if off == 0 { return Err(Error::PageNotFound); }
        off = directory(s, off, le, limits, budget)?.next;
        assert((p as int + 1) * (limits.max_tags as int + 1)
            == p as int * (limits.max_tags as int + 1) + limits.max_tags + 1) by(nonlinear_arith);
        p = p + 1;
    }
    if off == 0 { return Err(Error::PageNotFound); }
    proof { if eligible {
        traversal_budget(old(budget).remaining as int, budget.remaining as int,
            p as int, page as int, limits.max_tags as int,
            19 * limits.max_tags + 4 + (limits.max_segments as int)*(limits.max_input as int+1) + limits.max_output);
    } }
    let d = directory(s, off, le, limits, budget)?;
    assert(eligible ==> budget.remaining as int >=
        19 * limits.max_tags + 4 + (limits.max_segments as int)*(limits.max_input as int+1) + limits.max_output);
    let format=read_format(s,d,le,limits,budget)?;
    reveal(profile_fields);
    let w=format.width;let height=format.height;let channels=format.channels;
    let photo=format.photo;let alpha=format.alpha;let output_len=format.output_len;
    let packbits=format.compression==32773;let lzw=format.compression==5;
    let bilevel=format.depth==1;let lsb=format.fill==2;
    reveal(profile_layout);
    let geometry=read_geometry(s,d,le,format,limits,budget)?;
    let bw=geometry.block_width;let bh=geometry.block_height;let across=geometry.across;
    let offsets=geometry.offsets;let counts=geometry.counts;let tiled=geometry.tiled;
    let segments=geometry.counts.count;
    assert(geometry.positive());
    assert(eligible ==> geometry == profile_geometry(s@, d, le));
    proof {
        assert(bw as int*channels>0) by(nonlinear_arith)
            requires bw>0, channels>0;
    }
    let stride = if bilevel { (bw-1)/8+1 } else { mul(bw,channels)? };
    assert(stride as int == geometry.row_bytes());
    assert(eligible ==> budget.remaining as int >=
        (limits.max_segments as int)*(limits.max_input as int+1) + limits.max_output);
    let ghost before_segments = budget.remaining;
    assert(eligible ==> decode_base(geometry)<=usize::MAX);
    assert(decode_base(geometry)==output_len as int + (if alpha!=0 {output_len as int} else {0int}));
    let base_work=if alpha!=0 {add(output_len,output_len)?} else {output_len};
    assert(base_work as int==decode_base(geometry));
    let mut decode_work = base_work;
    proof { codec_work_zero(s@,geometry); }
    let mut j: usize = 0;
    while j < segments
        invariant j <= segments, bw > 0, bh > 0,
            eligible == (supported_profile(s@, page, *limits)
                && old(budget).remaining as int >= inspection_work_bound(page, *limits)),
            eligible ==> before_segments as int >= (limits.max_segments as int)*(limits.max_input as int+1) + limits.max_output,
            s.len() <= limits.max_input,
            segments <= limits.max_segments, stride as int == geometry.row_bytes(), stride > 0,
            geometry.positive(), geometry.tiled == tiled,
            base_work as int==decode_base(geometry),
            decode_work as int == base_work as int + (if packbits || lzw { codec_work_prefix(s@,geometry,j as nat) } else { 0int }),
            eligible ==> base_work as int + codec_work_prefix(s@,geometry,segments as nat) <= usize::MAX || !(packbits || lzw),
            forall|i: int| 0 <= i < j ==> segment_valid(s@,geometry,tiled,i),
            geometry == (Geometry { width: w, height, channels, alpha, block_width: bw,
                block_height: bh, across, offsets, counts, le, white: photo == 0, bilevel, lsb, packbits, lzw, tiled }),
            eligible ==> forall|i: int| 0 <= i < segments ==> segment_valid(s@, geometry, tiled, i),
            before_segments as int - budget.remaining <= j as int*(limits.max_input as int+1),
            budget.remaining <= old(budget).remaining,
        decreases segments - j,
    {
        assert(eligible ==> segment_valid(s@, geometry, tiled, j as int));
        reveal(segment_valid);
        proof { if eligible {
            assert(before_segments as int >= segments as int*(limits.max_input as int+1)+limits.max_output) by(nonlinear_arith)
                requires before_segments as int >= (limits.max_segments as int)*(limits.max_input as int+1)+limits.max_output,
                    segments <= limits.max_segments;
            traversal_budget(before_segments as int,budget.remaining as int,j as int,
                segments as int-1,limits.max_input as int,limits.max_output as int);
        } }
        let ghost step_work = budget.remaining;
        spend(budget, 1)?;
        let start = value(s, offsets, j, le)?;
        let count = value(s, counts, j, le)?;
        if !checked_range(s.len(), start, count) { return Err(Error::Truncated); }
        let stored_rows = if tiled { bh } else {
            let first_row = mul(j, bh)?;
            if first_row >= height { return Err(Error::Invalid); }
            if height - first_row < bh { height - first_row } else { bh }
        };
        let needed = if bilevel {
            mul((bw - 1) / 8 + 1, stored_rows)?
        } else {
            mul(mul(bw, stored_rows)?, channels)?
        };
        if packbits || lzw {
            if packbits { pb_validate(s,start,count,stride,needed,budget)?; }
            else { lz_validate(s,start,count,needed,budget)?; }
            proof { if eligible {
                codec_work_monotone(s@,geometry,(j+1) as nat,segments as nat);
            } }
            proof { codec_work_step(s@,geometry,j as nat); }
            assert(codec_work_prefix(s@,geometry,(j+1) as nat)
                == codec_work_prefix(s@,geometry,j as nat)+count+needed);
            decode_work = add(add(decode_work,count)?,needed)?;
        } else if count < needed { return Err(Error::Truncated); }
        assert(segment_valid(s@,geometry,tiled,j as int));
        assert(step_work as int-budget.remaining <= limits.max_input as int+1);
        assert((j as int+1)*(limits.max_input as int+1) == j as int*(limits.max_input as int+1)+limits.max_input+1) by(nonlinear_arith);
        j = j + 1;
    }
    // Whole-segment checks already establish every sample address. Keep the
    // historical conservative work charge, without rereading every pixel.
    proof {
        assert forall|i: int| 0 <= i < output_len implies sample_valid(s@, geometry, i) by {
            segment_covers_sample(s@, geometry, tiled, i);
        }
    }
    if !(packbits || lzw) {
        proof { if eligible {
            assert(budget.remaining >= output_len) by(nonlinear_arith)
                requires before_segments as int >= (limits.max_segments as int)*(limits.max_input as int+1)+limits.max_output,
                    before_segments as int-budget.remaining <= segments as int*(limits.max_input as int+1),
                    segments <= limits.max_segments, output_len <= limits.max_output;
        } }
        if budget.remaining<output_len {
            let remaining=budget.remaining;
            spend(budget,remaining)?;
            return Err(Error::Limit);
        }
        spend(budget,output_len)?;
    }
    Ok(Image { input: s, geometry, output_len, decode_work, directory: d, page })
}

}
