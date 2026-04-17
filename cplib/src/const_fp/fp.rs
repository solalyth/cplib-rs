use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Fp<const P: u32>(pub u32);

impl<const P: u32> Fp<P> {
    pub fn new(x: impl Into<i64>) -> Self {
        Self(x.into().rem_euclid(998244353) as u32)
    }
    
    pub fn pow(mut self, mut exp: usize) -> Self {
        let mut res = Self(1);
        while exp != 0 {
           if exp&1 == 1 { res = res * self; }
           self = self*self;
           exp >>= 1;
        }
        res
    }
    
    pub fn inv(self) -> Self {
        assert!(self.0 != 0);
        self.pow(P as usize-2)
    }
}

impl<const P: u32> Add for Fp<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let t = self.0 + rhs.0;
        Self(if P <= t {t-P} else {t})
    }
}

impl<const P: u32> Sub for Fp<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let t = self.0 + P - rhs.0;
        Self(if P <= t {t-P} else {t})
    }
}

impl<const P: u32> Mul for Fp<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 as u64 * rhs.0 as u64 % P as u64) as u32)
    }
}

impl<const P: u32> Div for Fp<P> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs.0 != 0);
        self * rhs.inv()
    }
}
