use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Point(pub i64, pub i64);

impl Point {
    /// 内積を計算する。反時計回りに [+, -, -, +] の符号になる。
    pub fn dot(self, other: Self) -> i64 {
        self.0*other.0 + self.1*other.1
    }
    
    /// 外積を計算する。反時計回りに [+, +, -, -] の符号になる。
    pub fn cross(self, other: Self) -> i64 {
        self.0*other.1 - self.1*other.0
    }
    
    pub fn abs(self) -> f64 {
        ((self.0 as f64).powi(2) + (self.1 as f64).powi(2)).sqrt()
    }
    
    /// 偏角ソートの比較関数。外積がオーバーフローしないことを仮定する。
    /// 
    /// # Panics
    /// 
    /// - if `(x, y) == (0, 0)` (debug only)
    /// 
    /// # References
    /// 
    /// - https://ngtkana.hatenablog.com/entry/2021/11/13/202103
    pub fn argcmp(self, other: Self) -> std::cmp::Ordering {
        debug_assert!(self != Point(0, 0) && other != Point(0, 0));
        ((self.1, self.0) < (0, 0)).cmp(&((other.1, other.0) < (0, 0))).then((other.0*self.1).cmp(&(self.0*other.1)))
    }
}


impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Point(self.0 + rhs.0, self.1 + rhs.1) }
}

impl Sub for Point {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output { Point(self.0 - rhs.0, self.1 - rhs.1) }
}

impl Mul<i64> for Point {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output { Point(self.0 * rhs, self.1 * rhs) }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.argcmp(*other)
    }
}

impl From<(i64, i64)> for Point {
    fn from(v: (i64, i64)) -> Self { Point(v.0, v.1) }
}
