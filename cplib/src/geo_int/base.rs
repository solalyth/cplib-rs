use std::ops::{Add, Mul, Sub};

type Int = i64;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Point { pub x: Int, pub y: Int }

impl Point {
    pub const fn new(x: Int, y: Int) -> Self { Self { x, y } }
    pub fn pair(self) -> [Int; 2] { [self.x, self.y] }
    
    /// 反時計回りに `[+, -, -, +]` の符号。
    pub fn dot(self, p: Self) -> Int { self.x*p.x + self.y*p.y }
    /// 反時計回りに `[+, +, -, -]` の符号。
    pub fn cross(self, p: Self) -> Int { self.x*p.y - self.y*p.x }
    
    pub fn abs2(self) -> Int { self.x.pow(2) + self.y.pow(2) }
    pub fn abs(self) -> f64 { ((self.x as f64).powi(2) + (self.y as f64).powi(2)).sqrt() }
    
    pub fn rot90(self) -> Self { Point::new(-self.y, self.x) }
    pub fn array(self, arr: [[Int; 2]; 2]) -> Self {
        Point::new(self.x * arr[0][0] + self.y * arr[0][1], self.x * arr[1][0] + self.y * arr[1][1])
    }
    
    /// `[0, 2π)` の偏角ソート。長さは区別しない。
    /// 
    /// # Panics
    /// 
    /// - if `p == (0, 0)` or `q == (0, 0)`
    /// 
    /// # References
    /// 
    /// - https://ngtkana.hatenablog.com/entry/2021/11/13/202103
    pub fn argcmp(p: Self, q: Self) -> std::cmp::Ordering {
        let ([px, py], [qx, qy]) = (p.pair(), q.pair());
        assert!(px|py != 0 && qx|qy != 0);
        ((py, px) < (0, 0)).cmp(&((qy, qx) < (0, 0))).then((qx*py).cmp(&(px*qy)))
    }
    
    /// `PQ` に対する `PR` の向き。時計, 直線上, 反時計の順番に `(-1, 0, 1)` を返す。
    pub fn ccw(self: Self, p: Self, q: Self) -> Int {
        assert!(self != p);
        (p-self).cross(q-self).signum()
    }
}

impl Add for Point { type Output = Self; fn add(mut self, rhs: Self) -> Self::Output { self.x += rhs.x; self.y += rhs.y; self } }
impl Sub for Point { type Output = Self; fn sub(mut self, rhs: Self) -> Self::Output { self.x -= rhs.x; self.y -= rhs.y; self } }
impl<T: Into<Int>> Mul<T> for Point { type Output = Self; fn mul(mut self, rhs: T) -> Self::Output { let rhs = rhs.into(); self.x *= rhs; self.y *= rhs; self } }



#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Line { pub a: Int, pub b: Int, pub c: Int }

impl Line {
    pub fn new(a: Int, b: Int, c: Int) -> Self { Line { a, b, c } }
    pub fn from_point(p: Point, q: Point) -> Self {
        Line { a: q.y-p.y, b: p.x-q.x, c: p.y*q.x - p.x*q.y }
    }
    
    pub fn is_parallel(self, l: Line) -> bool { self.a*l.b == self.b*l.a }
    pub fn cross_point(self, l: Line) -> Option<(Point, Int)> {
        if self.is_parallel(l) { return None; }
        let mut d = self.a*l.b - self.b*l.a;
        let mut p = Point::new(self.b*l.c - self.c*l.b, self.c*l.a - self.a*l.c);
        if d < 0 { d *= -1; p = p * -1; }
        Some((p, d))
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Segment(pub Point, pub Point);

impl Segment {
    pub fn into_line(self) -> Line { Line::from_point(self.0, self.1) }
    pub fn dir(self) -> Point { self.1 - self.0 }
    
    pub fn is_intersect(self, s: Segment) -> bool {
        let f = Point::ccw(self.0, self.1, s.0) * Point::ccw(self.0, self.1, s.1) <= 0;
        let g = Point::ccw(s.0, s.1, self.0) * Point::ccw(s.0, s.1, self.1) <= 0;
        f && g
    }
    
    pub fn is_parallel(self, s: Segment) -> bool {
        self.dir().cross(s.dir()) == 0
    }
}
