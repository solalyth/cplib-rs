pub use crate::cplib::geo_int::base::*;

/// y 座標が大きい部分の凸包を構築する。
pub fn upper_monotone_chain(mut ps: Vec<Point>) -> Vec<Point> {
    ps.sort_unstable();
    let mut k = 0;
    for i in 0..ps.len() {
        while 2 <= k && 0 <= ps[k-2].ccw(ps[k-1], ps[i]) { k -= 1; }
        ps[k] = ps[i];
        k += 1;
    }
    ps.truncate(k);
    ps.dedup_by(|p, q| if p.x == q.x { q.y = q.y.max(p.y); true } else { false });
    ps
}

pub fn lower_monotone_chain(mut ps: Vec<Point>) -> Vec<Point> {
    ps.sort_unstable();
    let mut k = 0;
    for i in 0..ps.len() {
        while 2 <= k && ps[k-2].ccw(ps[k-1], ps[i]) <= 0 { k -= 1; }
        ps[k] = ps[i];
        k += 1;
    }
    ps.truncate(k);
    ps.dedup_by(|p, q| if p.x == q.x { q.y = q.y.min(p.y); true } else { false });
    ps
}
