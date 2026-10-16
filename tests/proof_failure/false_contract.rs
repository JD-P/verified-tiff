use vstd::prelude::*;
verus! {
fn deliberately_false() -> (v: u8)
    ensures v == 1,
{ 0 }
}
