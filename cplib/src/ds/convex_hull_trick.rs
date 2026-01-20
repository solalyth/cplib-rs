use std::{borrow::Borrow, collections::BTreeSet, fmt::Debug};



#[derive(Clone, Copy)]
pub struct Line {
    a: i64,
    b: i64,
    r: I64
}

impl Line {
    fn new(a: i64, b: i64, r: i64) -> Self { Self { a, b, r: I64(r)} }
    fn ab(&self) -> (i64, i64) { (self.a, self.b) }
}

impl PartialEq for Line { fn eq(&self, other: &Self) -> bool { self.a == other.a } }
impl Eq for Line {}
impl PartialOrd for Line { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
impl Ord for Line { fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.a.cmp(&other.a) } }
impl Borrow<i64> for Line { fn borrow(&self) -> &i64 { &self.a } }
impl Borrow<I64> for Line { fn borrow(&self) -> &I64 { &self.r } }
impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, R={})", self.a, self.b, self.r.0)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct I64(i64);



/// 最大値を取る区間の区切れを返す。すなわち、`x < sep` ならば左が最大で、`sep <= x` ならば右が最大である。
pub fn sep_line(l: (i64, i64), r: (i64, i64)) -> i64 {
    debug_assert!(l.0 < r.0);
    (l.1 - r.1).div_euclid(r.0 - l.0)
}



/// 直線集合 `ax+b` に対して、任意の `x` における最大値を求めることができる。<br>
/// 直線 `(a, b)` やクエリ `x` について `-4e9 <= a, b, x <= 4e9` であることを要請する。
/// 
/// - 直線追加: amortized `O(log N)` (たぶん)
/// - max 回答クエリ: `O(log N)`
/// 
/// # Reference
/// 
/// - [クエリが整数の Convex Hull Trick の凸判定 (noshi91)](https://noshi91.hatenablog.com/entry/2021/03/23/200810)
///   + 最大値を求めるため、`max {ax+b >= cx+d}` の形になっている。
#[derive(Debug)]
pub struct ConvexHullTrick {
    pub set: BTreeSet<Line>
}

impl ConvexHullTrick {
    pub fn new() -> Self {
        Self { set: BTreeSet::from([Line::new(0, -4e18 as i64, i64::MAX)]) }
    }
    
    pub fn add_line(&mut self, a: i64, b: i64) {
        assert!(-4e18 as i64 <= a && a <= 4e18 as i64 && -4e18 as i64 <= b && b <= 4e18 as i64);
        
        let mut del = vec![];
        let mut l: Option<Line> = None;
        let mut qr = i64::MAX;
        
        for &lnx in self.set.range(..=a).rev() {
            if let Some(l) = &mut l {
                if lnx.r.0 < sep_line(l.ab(), (a, b)) { break; }
                del.push(lnx.a);
                *l = lnx;
            } else if lnx.a == a {
                if b <= lnx.b { return; }
                del.push(lnx.a);
            } else {
                if lnx.r.0 < sep_line(lnx.ab(), (a, b)) { return; }
                del.push(lnx.a);
                l = Some(lnx);
            }
        }
        
        for rnx in self.set.range(a+1..) {
            if sep_line((a, b), rnx.ab()) < rnx.r.0 { qr = sep_line((a, b), rnx.ab()); break; }
            del.push(rnx.a);
        }
        
        for a in del { self.set.remove(&a); }
        
        if let Some(mut l) = l {
            l.r.0 = sep_line(l.ab(), (a, b));
            self.set.insert(l);
        }
        
        self.set.insert(Line::new(a, b, qr));
    }
    
    pub fn query(&self, x: i64) -> i64 {
        let (a, b) = self.set.range(I64(x)..).next().unwrap().ab();
        a*x + b
    }
}
