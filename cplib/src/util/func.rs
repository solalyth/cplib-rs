use std::ops::{RangeBounds, Bound};



pub fn partition_point(pred: impl Fn(usize) -> bool) -> usize {
    let (mut ok, mut ng) = (0, 1);
    while pred(ng) { (ok, ng) = (ng, ng*2); }
    while ng-ok > 1 {
        let x = ok+(ng-ok)/2;
        if pred(x) { ok = x; } else { ng = x; }
    }
    ok
}

pub fn binary_search(low: usize, high: usize) -> Option<usize> {
    if 1 < high.wrapping_sub(low) { Some(low.wrapping_add(high)/2) } else { None }
}



/// [`RangeBounds`] を半開区間の境界値 `l..r` に変換する。[`Bound::Unbounded`] は `0..sup` まで広げる。
/// ただし、区間長が `0` であるときは `[0, 0]` を返す。
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



pub(crate) fn join(s: impl Iterator<Item = String>) -> Option<String> {
    let mut res = s.into_iter().fold(String::new(), |mut acc, e| { acc += &e; acc += ", "; acc });
    if res.is_empty() { return None; }
    res.truncate(res.len() - 2);
    Some(res)
}
