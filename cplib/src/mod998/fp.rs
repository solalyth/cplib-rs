use std::{fmt::{Debug, Display}, ops::{Add, Mul, Neg, Sub}};

pub const M: usize = 998244353;
pub const R: usize = (1<<32) % M;
pub const R2: usize = (R*R) % M;
const P: usize = 998244351;
const MASK: usize = (1<<32)-1;

const fn prod_r(x: usize) -> usize {
    prod_rinv(x * R2)
}

const fn prod_rinv(x: usize) -> usize {
    let t = (((x & MASK)*P & MASK)*M + x) >> 32;
    if t < M { t } else { t-M }
}



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fp(u32);

impl Fp {
    pub const fn new(x: usize) -> Self {
        Fp(prod_r(if x < 4599987193 { x } else { x%M }) as u32)
    }
    
    pub const fn mul(self, rhs: Self) -> Self { Fp(prod_rinv(self.0 as usize * rhs.0 as usize) as u32) }
    
    /// `x^0 == 1` としている。
    pub const fn pow(mut self, mut exp: usize) -> Self {
        let mut res = Fp::new(1);
        exp %= M-1;
        while exp != 0 {
            if exp&1 == 1 { res = Fp::mul(self, res); }
            self = Fp::mul(self, self);
            exp >>= 1;
        }
        res
    }
    
    #[track_caller]
    pub const fn inv(self) -> Self { assert!(self.0 != 0); self.pow(M-2) }
    
    pub const fn val(self) -> usize { prod_rinv(self.0 as usize) }
}

impl Add for Fp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { let t = (self.0 + rhs.0) as usize; Fp(if t < M { t } else { t-M } as u32) }
}

impl Sub for Fp {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { let t = (M as u32 + self.0 - rhs.0) as usize; Fp(if t < M { t } else { t-M } as u32) }
}

impl Mul for Fp {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output { Fp::mul(self, rhs) }
}

impl Neg for Fp {
    type Output = Self;
    fn neg(self) -> Self::Output { Fp(if self.0 == 0 {0} else {M as u32 - self.0}) }
}

impl Display for Fp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val()) }
}

impl Debug for Fp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val()) }
}


#[macro_export]
macro_rules! fp {
    ($x:expr) => {
        Fp::new(($x as i64 % 998244353 + 998244353) as usize)
    };
}
