use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vect(f64, f64);

impl Vect {
    pub fn abs2(self) -> f64 {
        self.0.powi(2) + self.1.powi(2)
    }
    
    pub fn abs(self) -> f64 { self.abs2().sqrt() }
    
    pub fn rot(self, rad: f64) -> Self {
        let sin = rad.sin();
        let cos = rad.cos();
        Vect(self.0*cos-self.1*sin, self.0*sin+self.1*cos)
    }
    
    pub fn decomp(self, v: Vect) -> [Vect; 2] {
        // v = kr
        // <self,r>*r = <self,v>*v / k^2
        // if abs2 = v.abs2();
        
        let res = v * ((self * v) / v.abs2());
        todo!()
    }
}

impl Neg for Vect { type Output = Vect; fn neg(self) -> Self::Output { Vect(-self.0, -self.1) } }
impl Add for Vect {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vect(self.0+rhs.0, self.1+rhs.1)
    }
}
impl Sub for Vect {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

// inner product
impl Mul for Vect {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.0*rhs.0 + self.1*rhs.1
    }
}

// scalar product
impl Mul<f64> for Vect {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vect(self.0*rhs, self.1*rhs)
    }
}
