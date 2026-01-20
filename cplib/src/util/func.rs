use std::ops::{RangeBounds, Bound};

/// [`RangeBounds`] を半開区間の境界値 `l..r` に変換する。[`Bound::Unbounded`] は `0..sup` まで広げる。
/// ただし、左端が `sup` を超えていたとしても、区間長が `0` であるときは `[0, 0]` を返す。
/// 
/// # Panics
/// 
/// - if not `range.r <= sup`
pub fn to_bounds(range: impl RangeBounds<usize>, sup: usize) -> [usize; 2] {
    let mut l = match range.start_bound() {
        Bound::Included(&v) => v,
        Bound::Excluded(&v) => v+1,
        Bound::Unbounded => 0
    };
    let mut r = match range.end_bound() {
        Bound::Included(&v) => v+1,
        Bound::Excluded(&v) => v,
        Bound::Unbounded => sup
    };
    
    if l >= r { l = 0; r = 0; }
    assert!(r <= sup, "valid: 0..{sup}, input: {l}..{r}");
    
    [l, r]
}



// pub(crate) fn join(s: impl Iterator<Item = String>) -> Option<String> {
//     let mut res = s.into_iter().fold(String::new(), |mut acc, e| { acc += &e; acc += ", "; acc });
//     if res.is_empty() { return None; }
//     res.truncate(res.len() - 2);
//     Some(res)
// }
