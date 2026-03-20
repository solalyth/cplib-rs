use std::ops::{RangeBounds, Bound};

/// [`RangeBounds`] を半開区間の境界値 `l..r` に変換する。変換するときに `0..sup` と共通区間を取る。区間長が `0` になるときは代わりに `[0, 0]` を返す。
pub fn to_bounds(range: impl RangeBounds<usize>, sup: usize) -> [usize; 2] {
    let l = match range.start_bound() {
        Bound::Included(&v) => v,
        Bound::Excluded(&v) => v+1,
        Bound::Unbounded => 0
    };
    let r = match range.end_bound() {
        Bound::Included(&v) => v+1,
        Bound::Excluded(&v) => v,
        Bound::Unbounded => sup
    }.min(sup);
    
    if l >= r { [0, 0] } else { [l, r] }
}


// static mut X: u64 = 0;

// pub fn rng_reset() {
//     unsafe {
//         use std::time::{SystemTime, UNIX_EPOCH};
//         X = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64 | 1;
//     }
// }

// pub fn rng_u64() -> u64 {
//     unsafe {
//         X ^= X << 13;
//         X ^= X >> 17;
//         X ^= X << 5;
//         X-1
//     }
// }

// pub fn rng_us() -> usize { rng_u64() as usize }
