/// Montgomery 乗算を行う構造体。
/// 
/// ここでは `R = 2^32` とし、法 `m` は `2^32` 以下の奇数である。
/// 
/// # References
/// 
/// + https://rsk0315.hatenablog.com/entry/2022/11/27/060616
///   - `N -> m, N' -> mont` と読み替えている。
#[derive(Debug)]
pub struct Montgomery {
    m: u64,
    /// `r2 = R^2 mod m`
    r2: u64,
    /// `mont = (R * R^{-1} - 1) / m` を満たす。特に `0 <= mont < R` である。
    mont: u32
}

impl Montgomery {
    /// # Panic
    /// 
    /// if not `m < 2^32 && m is odd`
    pub fn new(m: u64) -> Self {
        assert!(m < 1<<32 && m%2 == 1);
        
        let r2 = (!m+1) % m;
        let mut tmp = m as u32;
        for _ in 0..4 {
            tmp = tmp.wrapping_mul(2u32.wrapping_sub(tmp.wrapping_mul(m as u32)));
        }
        Self { m, r2, mont: !tmp+1 }
    }
    
    /// `a in [0, m)` について `aR` を求める。`res in [0, m)` が保証される。
    pub fn prod_r(&self, x: u64) -> u64 {
        self.prod_rinv(x * self.r2)
    }
    
    /// `a in [0, mR)` について `aR^{-1}` を求める。`res in [0, m)` が保証される。
    /// 
    /// 特に `m^2 < mR` であるから、`[0, m)` な値の積について計算できる。
    pub fn prod_rinv(&self, x: u64) -> u64 {
        let a = self.mont.wrapping_mul(x as u32) as u64;
        let b = (x + a*self.m) >> 32;
        if self.m <= b { b - self.m } else { b }
    }
    
    /// `mod m` 演算を行う。
    pub fn rem(&self, mut x: u64) -> u64 {
        if x <= self.m * 8 {
            while self.m <= x { x -= self.m; } x
        } else {
            self.prod_r(self.prod_rinv(x))
        }
    }
    
    /// `xR -> (x^n)R` を返す。
    pub fn pow(&self, mut xr: u64, mut n: u64) -> u64 {
        if n == 0 { return self.prod_r(1); }
        let mut res = 1<<32;
        while n != 0 {
            if n&1 == 1 { res = self.prod_rinv(res*xr); }
            xr = self.prod_rinv(xr*xr);
            n >>= 1;
        }
        res
    }
}
