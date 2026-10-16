use super::*;
verus! {
// SGILog32 byte-plane RLE. Packets cannot cross a row/plane boundary.
pub(super) open spec fn sg_len(s: Seq<u8>, at: int) -> int {
    let n = s[at] as int;
    if n < 128 { n } else { n - 126 }
}

pub(super) open spec fn sg_payload(s: Seq<u8>, at: int) -> int {
    if s[at] < 128 { sg_len(s, at) } else { 1 }
}

pub(super) open spec fn sg_next(s: Seq<u8>, at: int) -> int { at + 1 + sg_payload(s, at) }

pub(super) open spec fn sg_packet_ok(s: Seq<u8>, at: int, end: int) -> bool {
    0 <= at < end <= s.len() && sg_next(s, at) <= end
}

#[verifier::opaque]
pub(super) open spec fn sg_valid(s: Seq<u8>, at: int, end: int, stride: int, left: int) -> bool
    decreases end - at,
{
    0 <= at <= end <= s.len() && stride > 0 && left >= 0
    && if at == end { left == 0 } else {
        sg_packet_ok(s, at, end)
        && sg_len(s, at) <= left
        && sg_len(s, at) <= (if left % stride == 0 { stride } else { left % stride })
        && sg_valid(s, sg_next(s, at), end, stride, left - sg_len(s, at))
    }
}

#[verifier::opaque]
pub(super) open spec fn sg_byte(s: Seq<u8>, at: int, end: int, k: int) -> u8
    decreases end - at,
{
    if k < 0 || !sg_packet_ok(s, at, end) { 0u8 }
    else if k < sg_len(s, at) {
        if s[at] < 128 { s[at + 1 + k] } else { s[at + 1] }
    } else { sg_byte(s, sg_next(s, at), end, k - sg_len(s, at)) }
}

#[derive(Clone, Copy)]
struct Packet { data: usize, next: usize, len: usize, repeat: bool }

fn sg_packet(s: &[u8], at: usize, end: usize) -> (r: Result<Packet, Error>)
    requires end <= s.len(),
    ensures r.is_ok() == sg_packet_ok(s@, at as int, end as int),
        r.is_ok() ==> r.unwrap().data == at + 1,
        r.is_ok() ==> r.unwrap().next == sg_next(s@, at as int),
        r.is_ok() ==> at < r.unwrap().next <= end,
        r.is_ok() ==> r.unwrap().len == sg_len(s@, at as int),
        r.is_ok() ==> r.unwrap().len <= 129,
        r.is_ok() ==> r.unwrap().repeat == (s@[at as int] >= 128),
        r.is_ok() ==> (if r.unwrap().repeat { r.unwrap().data < end }
            else { r.unwrap().data + r.unwrap().len <= end }),
{
    if at >= end { return Err(Error::Truncated); }
    let n = s[at] as usize;
    let data = at + 1;
    let len = if n < 128 { n } else { n - 126 };
    let payload = if n < 128 { len } else { 1 };
    if payload > end - data { return Err(Error::Truncated); }
    Ok(Packet { data, next: data + payload, len, repeat: n >= 128 })
}

pub(super) fn sg_validate(s: &[u8], start: usize, count: usize, stride: usize,
    needed: usize, budget: &mut Budget) -> (r: Result<(), Error>)
    requires range_ok(s.len() as int, start as int, count as int), stride > 0,
    ensures final(budget).remaining <= old(budget).remaining,
        r.is_ok() == (sg_valid(s@, start as int, start as int + count, stride as int, needed as int)
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
            sg_valid(s@, start as int, end as int, stride as int, needed as int)
                == sg_valid(s@, at as int, end as int, stride as int, left as int),
        decreases end - at,
    {
        reveal_with_fuel(sg_valid, 1);
        let p = sg_packet(s, at, end)?;
        if p.len > left { return Err(Error::Invalid); }
        let row_left = if left % stride == 0 { stride } else { left % stride };
        if p.len > row_left { return Err(Error::Invalid); }
        left = left - p.len;
        at = p.next;
    }
    reveal_with_fuel(sg_valid, 1);
    if left != 0 { return Err(Error::Truncated); }
    Ok(())
}

// A streaming cursor retains one SGILog packet, never the expanded segment.
// Its ghost invariant relates the unread suffix to the original byte model.
#[derive(Clone, Copy)]
struct Cursor {
    ghost _start: usize,
    end: usize,
    ghost _stride: usize,
    ghost _needed: usize,
    at: usize, emitted: usize, data: usize, run: usize, repeat: bool,
}

impl Cursor {
    spec fn valid(self, s: Seq<u8>) -> bool {
        &&& self._start <= self.at <= self.end <= s.len()
        &&& self._stride > 0
        &&& self.emitted <= self._needed
        &&& self.run <= self._needed - self.emitted
        &&& (if self.repeat { self.run > 0 ==> self.data < self.end }
            else { self.data as int + self.run <= self.end })
        &&& sg_valid(s, self.at as int, self.end as int, self._stride as int,
            self._needed as int - self.emitted - self.run)
        &&& (forall|k: int| 0 <= k < self.run ==>
            #[trigger] sg_byte(s, self._start as int, self.end as int, self.emitted as int + k)
                == s[self.data as int + if self.repeat { 0int } else { k }])
        &&& (forall|k: int| self.run <= k < self._needed - self.emitted ==>
            #[trigger] sg_byte(s, self._start as int, self.end as int, self.emitted as int + k)
                == sg_byte(s, self.at as int, self.end as int, k - self.run))
    }

    proof fn run_byte(self, s: Seq<u8>, k: int)
        requires self.valid(s), 0 <= k < self.run,
        ensures sg_byte(s, self._start as int, self.end as int, self.emitted as int + k)
            == s[self.data as int + if self.repeat { 0int } else { k }],
    {}

    proof fn tail_byte(self, s: Seq<u8>, k: int)
        requires self.valid(s), self.run <= k < self._needed - self.emitted,
        ensures sg_byte(s, self._start as int, self.end as int, self.emitted as int + k)
            == sg_byte(s, self.at as int, self.end as int, k - self.run),
    {}

    fn new(_s: &[u8], start: usize, count: usize, _stride: usize, _needed: usize) -> (r: Self)
        requires range_ok(_s.len() as int, start as int, count as int), _stride > 0,
            sg_valid(_s@, start as int, start as int + count, _stride as int, _needed as int),
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
            v == sg_byte(s@, old(self)._start as int, old(self).end as int, old(self).emitted as int),
    {
        while self.run == 0
            invariant self.valid(s@), self.emitted < self._needed,
                self._start == old(self)._start, self.end == old(self).end,
                self._stride == old(self)._stride, self._needed == old(self)._needed,
                self.emitted == old(self).emitted, self.at >= old(self).at,
            decreases self.end - self.at,
        {
            reveal_with_fuel(sg_valid, 1);
            let ghost before = *self;
            let packet = sg_packet(s, self.at, self.end);
            assert(packet.is_ok());
            let p = match packet { Ok(p) => p, Err(_) => { return 0; } };
            proof {
                assert forall|k: int| 0 <= k < p.len implies
                    #[trigger] sg_byte(s@, self._start as int, self.end as int, self.emitted as int + k)
                        == s@[p.data as int + if p.repeat { 0int } else { k }] by {
                    reveal_with_fuel(sg_byte, 1);
                }
                assert forall|k: int| p.len <= k < self._needed - self.emitted implies
                    #[trigger] sg_byte(s@, self._start as int, self.end as int, self.emitted as int + k)
                        == sg_byte(s@, p.next as int, self.end as int, k - p.len) by {
                    reveal_with_fuel(sg_byte, 1);
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
                #[trigger] sg_byte(s@, self._start as int, self.end as int, self.emitted as int + k)
                    == s@[self.data as int + if self.repeat { 0int } else { k }] by {
                assert(0 <= k + 1 < before.run);
                before.run_byte(s@, k + 1);
            }
            assert forall|k: int| self.run <= k < self._needed - self.emitted implies
                #[trigger] sg_byte(s@, self._start as int, self.end as int, self.emitted as int + k)
                    == sg_byte(s@, self.at as int, self.end as int, k - self.run) by {
                assert(before.run <= k + 1 < before._needed - before.emitted);
                before.tail_byte(s@, k + 1);
            }
        }
        v
    }
}


pub(super) fn decode(s:&[u8],start:usize,count:usize,stride:usize,needed:usize,out:&mut [u8])
    requires range_ok(s.len() as int,start as int,count as int),stride>0,
        sg_valid(s@,start as int,start as int+count,stride as int,needed as int),needed<=old(out).len(),
    ensures final(out).len()==old(out).len(),
        forall|i:int| 0<=i<needed ==> final(out)@[i]==sg_byte(s@,start as int,start as int+count,i),
        forall|i:int| needed<=i<old(out).len() ==> final(out)@[i]==old(out)@[i],
{
    let mut cursor=Cursor::new(s,start,count,stride,needed);let mut i=0usize;
    while i<needed
        invariant cursor.valid(s@),cursor._start==start,cursor.end==start+count,cursor._stride==stride,
            cursor._needed==needed,cursor.emitted==i,i<=needed<=out.len(),out.len()==old(out).len(),
            forall|k:int| 0<=k<i ==> out@[k]==sg_byte(s@,start as int,start as int+count,k),
            forall|k:int| needed<=k<out.len() ==> out@[k]==old(out)@[k],
        decreases needed-i,
    {out[i]=cursor.next(s);i=i+1;}
}
}
