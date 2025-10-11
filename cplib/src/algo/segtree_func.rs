//! セグメント木状の列を管理するための関数群。
//! 
//! - 1-indexed
//! - 深さを `d` とすると、全体の長さは `1 << d+1` であり、最下層の長さは `1 << d` である。
//! - 遅延対象は自身を含まない。
//! 
//! # Usage
//! 
//! fold: (`push` lazy) → `seg_idx` value<br>
//! apply: `push` lazy → `seg_idx` lazy → `update` value

use std::ops::RangeBounds;
use crate::cplib::util::func::to_bounds;



pub fn push(d: usize, range: impl RangeBounds<usize>) -> Vec<usize> {
    let [l, r] = to_bounds(range, 1<<d).map(|v| v+(1<<d));
    (1..d).rev().map(|d| l >> d).chain((1..d).rev().map(|d| (r-1) >> d)).collect()
}

pub fn seg_idx(d: usize, range: impl RangeBounds<usize>) -> Vec<usize> {
    let [mut l, mut r] = to_bounds(range, 1<<d).map(|v| v+(1<<d));
    let (mut lv, mut rv) = (vec![], vec![]);
    while l < r {
        if l&1 == 1 { lv.push(l); l += 1; }
        if r&1 == 1 { rv.push(r-1); }
        l >>= 1; r >>= 1;
    }
    lv.extend(rv.into_iter().rev());
    lv
}

pub fn update(d: usize, range: impl RangeBounds<usize>) -> Vec<usize> {
    let [l, r] = to_bounds(range, 1<<d).map(|v| v+(1<<d));
    let mut res = vec![];
    for i in 1..d {
        if ((l >> i) << i) != l { res.push(l >> i); }
        if ((r >> i) << i) != r { res.push(r-1 >> i); }
    }
    res
}
