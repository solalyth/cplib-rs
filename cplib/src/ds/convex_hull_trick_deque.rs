use std::collections::VecDeque;


/// 最大値を取る区間の区切れを返す。すなわち、`x in [-inf, sep)` ならば左が最大で、`[sep, inf)` ならば右が最大である。
fn sep(l: (i64, i64), r: (i64, i64)) -> i64 {
    debug_assert!(l.0 < r.0);
    (l.1 - r.1).div_euclid(r.0 - l.0)
}



/// 一次関数の集合に対して、点 `x` における最大値を求めることができる。
/// 
/// - 直線追加 (push_front/push_back): amortized `O(1)`
/// - point max クエリ: `O(log N)`
/// 
/// # Reference
/// 
/// - [クエリが整数の Convex Hull Trick の凸判定 (noshi91)](https://noshi91.hatenablog.com/entry/2021/03/23/200810)
///   + 最大値を求めるため、`max {ax+b >= cx+d}` の形になっている。
#[derive(Debug)]
pub struct ConvexHullTrick {
    dat: VecDeque<(i64, i64)>,
}

impl ConvexHullTrick {
    pub fn new() -> Self {
        Self { dat: VecDeque::new() }
    }
    
    /// `a` は末尾の傾き以上である必要がある。
    pub fn push_back(&mut self, a: i64, b: i64) {
        if let Some(r) = self.dat.back() {
            assert!(r.0 <= a);
            if r.0 == a {
                if b <= r.1 { return; }
                self.dat.pop_back();
            }
        }
        
        while self.dat.len() >= 2 {
            let l = self.dat[self.dat.len()-2];
            let r = self.dat[self.dat.len()-1];
            if sep(l, r) < sep(r, (a, b)) { break; }
            self.dat.pop_back();
        }
        self.dat.push_back((a, b));
    }
    
    pub fn push_front(&mut self, a: i64, b: i64) {
        if let Some(l) = self.dat.front() {
            assert!(a <= l.0);
            if l.0 == a {
                if b <= l.1 { return; }
                self.dat.pop_front();
            }
        }
        
        while self.dat.len() >= 2 {
            let l = self.dat[1];
            let r = self.dat[0];
            if sep((a, b), l) < sep(l, r) { break; }
            self.dat.pop_front();
        }
        self.dat.push_front((a, b));
    }
    
    pub fn query(&self, x: i64) -> i64 {
        let (mut ok, mut ng) = (0, self.dat.len());
        while ng-ok > 1 {
            let i = (ok+ng)/2;
            if x < sep(self.dat[i-1], self.dat[i]) { ng = i; } else { ok = i; }
        }
        let (a, b) = self.dat[ok];
        a*x + b
    }
}
