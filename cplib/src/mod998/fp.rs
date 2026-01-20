use std::{fmt::{Debug, Display}, ops::{Add, Mul, Neg, Sub}};

pub const M: usize = 998244353;
pub const R: usize = (1<<32) % M;
pub const R2: usize = (R*R) % M;
const P: usize = 998244351;
const MASK: usize = (1<<32)-1;


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fp(usize);

impl Fp {
    pub const fn new(x: usize) -> Self {
        Fp::prod_r(x % M)
    }
    
    /// `x < 4.6 * 998244353` くらいなら可能。
    pub const fn raw(x: usize) -> Self {
        assert!(x < 4599987193);
        Fp::prod_r(x)
    }
    
    pub const fn mul(self, rhs: Self) -> Self {
        Fp::prod_rinv(self.0 * rhs.0)
    }
    
    /// `x^0 == 1` としている。
    pub const fn pow(mut self, mut exp: usize) -> Self {
        if exp == 0 { return Fp::new(1); }
        let mut res = self;
        exp = (exp-1) % (M-1);
        while exp != 0 {
            if exp & 1 == 1 {
                res = Fp::mul(self, res);
            }
            self = Fp::mul(self, self);
            exp >>= 1;
        }
        res
    }
    
    pub const fn inv(self) -> Self { assert!(self.0 != 0); self.pow(M-2) }
    
    pub const fn val(self) -> usize { Fp::prod_rinv(self.0).0 }
    
    
    
    const fn prod_rinv(x: usize) -> Self {
        let t = (((x & MASK)*P & MASK)*M + x) >> 32;
        Fp(if t < M { t } else { t-M })
    }
    
    const fn prod_r(x: usize) -> Self {
        Fp::prod_rinv(x * R2)
    }
}

impl Add for Fp {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let t = self.0 + rhs.0;
        Fp(if t < M { t } else { t-M })
    }
}

impl Sub for Fp {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let t = self.0 + M - rhs.0;
        Fp(if t < M { t } else { t-M })
    }
}

impl Mul for Fp {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Fp::mul(self, rhs)
    }
}

impl Neg for Fp {
    type Output = Self;
    fn neg(self) -> Self::Output {
        if self.0 == 0 { self } else { Fp(M - self.0) }
    }
}

impl Display for Fp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val())
    }
}

impl Debug for Fp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val())
    }
}
