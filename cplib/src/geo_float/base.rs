use std::ops::{Add, Mul, Sub};

type F = f64;
const EPS: F = 1e-9;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Point { pub x: F, pub y: F }

impl Point {
    pub const fn new(x: F, y: F) -> Self { Self { x, y } }
    pub fn pair(self) -> [F; 2] { [self.x, self.y] }
    
    /// 反時計回りに `[+, -, -, +]` の符号。
    pub fn dot(self, p: Self) -> F { self.x*p.x + self.y*p.y }
    /// 反時計回りに `[+, +, -, -]` の符号。
    pub fn cross(self, p: Self) -> F { self.x*p.y - self.y*p.x }
    
    pub fn abs2(self) -> F { self.x.powi(2) + self.y.powi(2) }
    pub fn abs(self) -> f64 { self.x.hypot(self.y) }
    
    pub fn rot90(self) -> Self { Point::new(-self.y, self.x) }
    pub fn array(self, arr: [[F; 2]; 2]) -> Self {
        Point::new(self.x * arr[0][0] + self.y * arr[0][1], self.x * arr[1][0] + self.y * arr[1][1])
    }
    
    /// `[0, 2π)` の偏角ソート。長さは区別しない。
    /// 
    /// # Panics
    /// 
    /// - if `p == (0., 0.)` or `q == (0., 0.)`
    /// 
    /// # References
    /// 
    /// - https://ngtkana.hatenablog.com/entry/2021/11/13/202103
    pub fn argcmp(p: Self, q: Self) -> std::cmp::Ordering {
        let ([px, py], [qx, qy]) = (p.pair(), q.pair());
        assert!((px != 0. && py != 0.) && (qx != 0. && qy != 0.));
        ((py, px) < (0., 0.)).cmp(&((qy, qx) < (0., 0.))).then((qx*py - px*qy).partial_cmp(&0.).unwrap())
    }
    
    /// `PQ` に対する `PR` の向き。時計, 直線上, 反時計の順番に `(-1, 0, 1)` を返す。
    pub fn ccw(self: Self, p: Self, q: Self) -> F {
        (p-self).cross(q-self).signum()
    }
}

impl Add for Point { type Output = Self; fn add(mut self, rhs: Self) -> Self::Output { self.x += rhs.x; self.y += rhs.y; self } }
impl Sub for Point { type Output = Self; fn sub(mut self, rhs: Self) -> Self::Output { self.x -= rhs.x; self.y -= rhs.y; self } }
impl<T: Into<F>> Mul<T> for Point { type Output = Self; fn mul(mut self, rhs: T) -> Self::Output { let rhs = rhs.into(); self.x *= rhs; self.y *= rhs; self } }



#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Line { pub a: F, pub b: F, pub c: F }

impl Line {
    pub fn new(a: F, b: F, c: F) -> Self { Line { a, b, c } }
    pub fn from_point(p: Point, q: Point) -> Self {
        Line { a: q.y-p.y, b: p.x-q.x, c: p.y*q.x - p.x*q.y }
    }
    
    pub fn is_parallel(self, l: Line) -> bool { self.a*l.b == self.b*l.a }
    pub fn cross_point(self, l: Line) -> Option<Point> {
        if self.is_parallel(l) { return None; }
        let d = self.a*l.b - self.b*l.a;
        let p = Point::new(self.b*l.c - self.c*l.b, self.c*l.a - self.a*l.c) * (1./d);
        Some(p)
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
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
