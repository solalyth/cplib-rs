use std::{fmt::{Debug, Display}, ops::{Add, Div, Mul, Neg, Sub}};

pub type Fp998 = Fp<998244353>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fp<const P: u32>(pub u32);

impl<const P: u32> Fp<P> {
    pub const fn raw(x: u32) -> Self {
        assert!(x < 2*P);
        Fp(if x < P { x } else { x-P })
    }
    
    pub fn new(x: impl Into<i64>) -> Self {
        Self(x.into().rem_euclid(P as i64) as u32)
    }
    
    pub const fn val(self) -> u32 { self.0 }
    
    pub const fn _mul(self, x: Self) -> Self {
        Self((self.0 as u64 * x.0 as u64 % P as u64) as u32)
    }
    
    const fn _pow(mut t: Self, mut x: Self, mut exp: usize) -> Self {
        while exp != 0 {
            if exp&1 == 1 { t = t._mul(x); }
            x = x._mul(x);
            exp >>= 1;
        }
        t
    }
    
    pub const fn pow(self, exp: usize) -> Self {
        Self::_pow(Self(1), self, exp)
    }
    
    pub const fn inv(self) -> Self {
        assert!(self.0 != 0);
        self.pow(P as usize-2)
    }
}

macro_rules! a {
    ($t:ty, $($f:tt)*) => {
        impl<const P: u32> $t for Fp<P> { type Output = Self; $($f)* }
    };
}

a!(Add, fn add(self, r: Self) -> Self { Fp::raw(self.0+r.0) });
a!(Sub, fn sub(self, r: Self) -> Self { Fp::raw(P+self.0-r.0) });
a!(Neg, fn neg(self) -> Self { Self(P-self.0) });
a!(Mul, fn mul(self, r: Self) -> Self { Self((self.0 as u64 * r.0 as u64 % P as u64) as u32) });
a!(Div, fn div(self, r: Self) -> Self { assert!(r.0 != 0); self * r.inv() });

impl<const P: u32> Display for Fp<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const P: u32> Debug for Fp<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fp({})", self.0)
    }
}
