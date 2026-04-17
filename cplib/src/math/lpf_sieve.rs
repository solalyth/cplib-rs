pub use crate::cplib::math::func::divisors;
use std::ops::{Add, Sub};

const MASK: usize = (1<<16)-1;

/// Least Prime Factor (lpf) を線形オーダーで計算する。
/// 
/// # References
/// 
/// - [線形篩で遊ぼう - rsk0315](https://rsk0315.hatenablog.com/entry/2024/08/25/194341)
pub struct LpfSieve {
    primes: Vec<usize>,
    /// `table[n]` = (`n/p^e: u32`, `e: u16`, `p: u16`)
    table: Vec<usize>,
}

impl LpfSieve {
    /// `max = 2^32` くらいまで可能。
    pub fn new(mut max: usize) -> Self {
        assert!(max <= 1e7 as usize);
        
        max = max.max(10);
        let mut primes = vec![];
        let mut table = vec![0; max+1];
        
        for i in 2..=max {
            let lpf = if table[i] == 0 {
                primes.push(i);
                // (1, 1, 0)
                table[i] = (1<<32) + (1<<16);
                i
            } else {
                table[i] & MASK
            };
            for &p in &primes {
                if !(p <= lpf && i*p <= max) { break; }
                // lpf(i) == p => (table[i], table[i]+1, p)
                // lpf(i) != p => (i, 1, p)
                table[i*p] = if lpf == p { table[i] + (1<<16) | p } else { (i<<32) + (1<<16) + p };
            }
        }
        
        // for n in 2..=max {
        //     let (p, e, nx) = if table[n] & MASK == 0 {
        //         (n, 1, 1)
        //     } else {
        //         (table[n] & MASK, table[n]>>16 & MASK, table[n]>>32)
        //     };
            
        //     crate::epr!("{n} -> p={p}, e={e}, nx={nx}");
        // }
        
        // crate::epr!("next = {:?}", (0..=max).map(|i| table[i]>>32 & MASK).collect::<Vec<_>>());
        // crate::epr!("lpf = {:?}", (0..=max).map(|i| table[i] & MASK).collect::<Vec<_>>());
        
        Self { primes, table }
    }
    
    pub fn max(&self) -> usize { self.table.len()-1 }
    
    pub fn primes(&self) -> &[usize] { &self.primes }
    
    pub fn is_prime(&self, n: usize) -> bool { 2 <= n && self.table[n] & MASK == 0 }
    
    pub fn lpf(&self, n: usize) -> usize {
        assert!(2 <= n);
        let t = self.table[n] & MASK;
        if t == 0 { n } else { t }
    }
    
    /// `(p, e, n/p^e)` を返す。
    /// 
    /// # Panics
    /// 
    /// - if not `n in [2, max]`
    fn data(&self, n: usize) -> (usize, usize, usize) {
        debug_assert!(2 <= n);
        if self.table[n] & MASK == 0 {
            (n, 1, 1)
        } else {
            (self.table[n] & MASK, self.table[n]>>16 & MASK, self.table[n]>>32)
        }
    }
    
    /// `n` による `(p, exp)` の列について `|res| f(&mut res, p, exp)` で fold した値を返す。
    /// 
    /// # Panics
    /// 
    /// - if not `n in [1, max]`
    pub fn fold<T>(&self, mut n: usize, mut init: T, mut vpe: impl FnMut(&mut T, usize, usize)) -> T {
        assert!(n != 0);
        while n != 1 {
            let (lpf, exp, nx) = self.data(n);
            vpe(&mut init, lpf, exp);
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
    pub fn fact(&self, mut n: usize) -> Vec<(usize, usize)> {
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
    
    
    /// Euler's totient function `φ(n)` を計算する。
    /// 
    /// # Panics
    /// 
    /// - if not `n in [1, self.max]`
    pub fn totient_point(&self, n: usize) -> usize {
        self.fold(n, n, |v, p, _| { *v -= *v/p; })
    }
    
    /// Euler's totient function `φ(i)` の `..=n` までのテーブルを計算する。ただし `res[0] == 0` となっている。
    pub fn totient(&self, n: usize) -> Vec<usize> {
        let mut res = vec![0; n+1];
        res[1] = 1;
        for i in 2..=n {
            let (lpf, exp, _nx) = self.data(i);
            res[i] = res[i/lpf] * if exp == 1 { lpf-1 } else { lpf };
        }
        res
    }
    
    
    
    /// Mobius function `μ(n)` を計算する。
    /// 
    /// メビウスの反転公式: `F(n) = \sum_{d|n} f(n)` であるとき、`f(n) = \sum_{d|n} F(d) μ(n/d)` が成立する。
    pub fn mobius_point(&self, n: usize) -> i64 {
        self.fold(n, 1, |v, _, e| {
            *v = if e == 1 { -*v } else { 0 };
        })
    }
    
    /// Mobius function `μ(i)` の `..=n` までのテーブルを計算する。ただし `res[0] == 0` となっている。
    pub fn mobius(&self, n: usize) -> Vec<i64> {
        let mut res = vec![0; n+1];
        res[1] = 1;
        for i in 2..=n {
            let (_, exp, nx) = self.data(i);
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
}
