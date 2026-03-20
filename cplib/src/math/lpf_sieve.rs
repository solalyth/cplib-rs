pub use crate::cplib::math::func::divisors;
use std::ops::{Add, Sub};

const MASK_29BIT: usize = (1<<29)-1;

/// Least Prime Factor (lpf) を線形オーダーで計算する。
/// 
/// # Memo
/// 
/// 素因数分解における素数の種類数は `max < 9.7e6` くらいの範囲で高々 7 種である。
/// 
/// # References
/// 
/// - [線形篩で遊ぼう - rsk0315](https://rsk0315.hatenablog.com/entry/2024/08/25/194341)
pub struct LpfSieve {
    primes: Vec<usize>,
    /// `table[i]` = (`exp` as 6bit, `lpf` as 29bit, `i/(lpf^exp)` as 29bit)
    table: Vec<usize>,
}

impl LpfSieve {
    /// 初期化する。`max = 2^23, 10^7` くらいまで可能。
    pub fn new(mut max: usize) -> Self {
        assert!(max <= 1e7 as usize);
        
        max = max.max(10);
        let mut primes = vec![];
        let mut table = vec![0; max+1];
        
        for i in 2..=max {
            if table[i] == 0 { primes.push(i); table[i] = (1 << 58) + (i << 29) + 1; }
            let lpf_i = (table[i] >> 29) & MASK_29BIT;
            for &p in &primes {
                if !(p <= lpf_i && i*p <= max) { break; }
                table[p*i] = if p == lpf_i { table[i] + (1 << 58) } else { (1 << 58) + (p << 29) + i };
            }
        }
        
        Self { primes, table }
    }
    
    pub fn max(&self) -> usize { self.table.len()-1 }
    
    pub fn primes(&self) -> &[usize] { &self.primes }
    
    pub fn is_prime(&self, n: usize) -> bool { (self.table[n] >> 29) & MASK_29BIT == n }
    
    
    /// `(lpf, exp, n/(lpf^exp))` を返す。
    /// 
    /// # Panics
    /// 
    /// - if not `n in [2, max]`
    fn data(&self, n: usize) -> (usize, usize, usize) {
        debug_assert!(2 <= n);
        ((self.table[n] >> 29) & MASK_29BIT, self.table[n] >> 58, self.table[n] & MASK_29BIT)
    }
    
    /// `n` による `(p, exp)` の列について `|res| f(&mut res, p, exp)` で fold した値を返す。
    /// 
    /// # Panics
    /// 
    /// - if not `n in [1, max]`
    pub fn fold<T>(&self, mut n: usize, mut init: T, mut f: impl FnMut(&mut T, usize, usize)) -> T {
        assert!(n != 0);
        while n != 1 {
            let (lpf, exp, nx) = self.data(n);
            f(&mut init, lpf, exp);
            n = nx;
        }
        init
    }
    
    /// 正整数 `n` を素因数分解した結果 `(p, exp)` の列を返す。`factorize(1) == []` である。
    /// 
    /// `n <= max` であるときは前計算した lpf テーブルで計算する。`n <= max^2` であるときは試し割りで計算する。
    /// 
    /// # Panics
    /// 
    /// - if not `n in [1, max^2]`
    pub fn factorize(&self, mut n: usize) -> Vec<(usize, usize)> {
        assert!(1 <= n && n <= self.max().pow(2));
        if n <= self.max() {
            self.fold(n, vec![], |v, p, e| { v.push((p, e)); })
        } else {
            let mut res = vec![];
            for &p in &self.primes {
                let mut cnt = 0;
                while n%p == 0 { cnt += 1; n /= p; }
                if cnt != 0 { res.push((p, cnt)); }
                if n < p*p { break; }
            }
            if n != 1 { res.push((n, 1)); }
            res
        }
    }
    
    
    // /// Euler's totient function `φ(n)` を計算する。
    // /// 
    // /// # Panics
    // /// 
    // /// - if not `n in [1, self.max]`
    // pub fn totient_point(&self, n: usize) -> usize {
    //     self.fold(n, n, |v, p, _e| { *v -= *v/p; })
    // }
    
    // /// Euler's totient function `φ(i)` の `..=n` までのテーブルを計算する。ただし `res[0] == 0` となっている。
    // pub fn totient(&self, n: usize) -> Vec<usize> {
    //     let mut res = vec![0; n+1];
    //     res[1] = 1;
    //     for i in 2..=n {
    //         let (lpf, exp, _nx) = self.data(i);
    //         res[i] = res[i/lpf] * if exp == 1 { lpf-1 } else { lpf };
    //     }
    //     res
    // }
    
    
    
    /// Mobius function `μ(n)` を計算する。
    /// 
    /// メビウスの反転公式: `F(n) = \sum_{d|n} f(n)` であるとき、`f(n) = \sum_{d|n} F(d) μ(n/d)` が成立する。
    pub fn mobius_point(&self, n: usize) -> i64 {
        self.fold(n, 1, |v, _p, e| {
            *v = if e == 1 { -*v } else { 0 };
        })
    }
    
    /// Mobius function `μ(i)` の `..=n` までのテーブルを計算する。ただし `res[0] == 0` となっている。
    pub fn mobius(&self, n: usize) -> Vec<i64> {
        let mut res = vec![0; n+1];
        res[1] = 1;
        for i in 2..=n {
            let (_lpf, exp, nx) = self.data(i);
            if exp == 1 { res[i] = -res[nx]; }
        }
        res
    }
    
    
    /// 約数についての和 `g(n) = sum_{n%i = 0} f(i)` を計算する。`O(NloglogN)`
    /// 
    /// lcm 畳み込みについて `zf[i] * zg[i] = z(lcm_convolution(f,g))[i]` が成立する。
    pub fn div_zeta<T: Copy + Add<Output = T>>(&self, mut f: Vec<T>) -> Vec<T> {
        let vl = f.len()-1;
        for &p in self.primes() {
            if vl < p { break; }
            for i in 1..=vl/p { f[i*p] = f[i*p] + f[i]; }
        }
        f
    }
    
    /// `div_zeta` の逆変換を行う。すなわち、`g(n) = sum_{n%i = 0} f(i)` である `g` から `f` を計算する。`O(NloglogN)`
    pub fn div_mobius<T: Copy + Sub<Output = T>>(&self, mut g: Vec<T>) -> Vec<T> {
        let vl = g.len()-1;
        for &p in self.primes() {
            if vl < p { break; }
            for i in (1..=vl/p).rev() { g[i*p] = g[i*p] - g[i]; }
        }
        g
    }
    
    // pub fn div_mobius_point<T: Default + Copy + Add<Output = T> + Sub<Output = T>>(&self, g: &[T], idx: usize) -> T {
    //     let mut res = T::default();
    //     for i in self.fold(idx, vec![1i32], |v, p, _| { for i in 0..v.len() { v.push(-v[i] * p as i32); } }) {
    //         // f(idx) = sum_{idx%i = 0} μ(i) g(idx/i)
    //         if 0 <= i { res = res + g[idx/i as usize]; } else { res = res - g[idx/-i as usize]; }
    //     }
    //     res
    // }
    
    /// 倍数についての和 `g(n) = sum_{i%n = 0} f(i)` を計算する。`O(NloglogN)`
    /// 
    /// gcd 畳み込みについて `zf[i] * zg[i] = z(gcd_convolution(f, g))[i]` が成立する。
    pub fn mul_zeta<T: Copy + Add<Output = T>>(&self, mut f: Vec<T>) -> Vec<T> {
        let vl = f.len() - 1;
        for &p in self.primes() {
            if vl < p { break; }
            for i in (1..=vl/p).rev() { f[i] = f[i] + f[i*p]; }
        }
        f
    }
    
    /// `mul_zeta` の逆変換を行う。すなわち、`g(n) = sum_{i%n = 0} f(i)` である `g` から `f` を計算する。`O(NloglogN)`
    pub fn mul_mobius<T: Copy + Sub<Output = T>>(&self, mut g: Vec<T>) -> Vec<T> {
        let vl = g.len() - 1;
        for &p in self.primes() {
            if vl < p { break; }
            for i in 1..=vl/p { g[i] = g[i] - g[i*p]; }
        }
        g
    }
    
    // pub fn mul_mobius_point<T: Default + Copy + Add<Output = T> + Sub<Output = T>>(&self, g: &[T], idx: usize) -> T {
    //     let mut res = T::default();
    //     let gl = g.len() - 1;
    //     let t = self.mobius(gl/idx);
    //     for (i, m) in t.into_iter().enumerate() {
    //         if m == 1 { res = res + g[i*idx]; } else if m == -1 { res = res - g[i*idx]; }
    //     }
    //     res
    // }
}
