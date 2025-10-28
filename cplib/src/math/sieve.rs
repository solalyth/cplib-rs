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
        // assert!(max <= 10_000_000);
        
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
    pub fn data(&self, n: usize) -> (usize, usize, usize) {
        assert!(2 <= n);
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
    
    /// 正整数 `n` を素因数分解した結果 `(p, exp)` の列を返す。`factorize_big(1) == vec![]` である。
    /// 
    /// 前計算した lpf テーブルで計算する。
    /// 
    /// # Panics
    /// 
    /// - if not `n in [1, max]`
    pub fn factorize(&self, n: usize) -> Vec<(usize, usize)> {
        self.fold(n, vec![], |v, p, e| { v.push((p, e)); })
    }
    
    /// 正整数 `n` を素因数分解した結果 `(p, exp)` の列を返す。`factorize_big(1) == vec![]` である。
    /// 
    /// 前計算した素数について試し割りを行う。
    /// 
    /// # Panics
    /// 
    /// - if not `n in [1, max^2]`
    pub fn factorize_big(&self, mut n: usize) -> Vec<(usize, usize)> {
        assert_ne!(n, 0);
        assert!(n <= self.max().pow(2));
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
    
    /// 約数を返す。昇順かは保証されていない。
    /// 
    /// 個数は実用範囲内で `N^(1/3)` 個程度である。(ref. [競プロにおける約数の個数の見積もり - noshi91](https://noshi91.hatenablog.com/entry/2022/07/05/021040))
    pub fn divisors(&self, n: usize) -> Vec<usize> {
        assert!(n != 0);
        self.fold(n, vec![1], |v, p, e| {
            for i in 0..v.len() {
                let mut k = 1;
                for _ in 0..e { k *= p; v.push(v[i]*k); }
            }
        })
    }
    
    
    
    /// Euler's totient function `φ(n)` を計算する。
    /// 
    /// # Panics
    /// 
    /// - if not `n in [1, self.max]`
    pub fn totient_point(&self, n: usize) -> usize {
        self.fold(n, n, |v, p, _e| { *v -= *v/p; })
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
    
    pub fn div_mobius_point<T: Default + Copy + Add<Output = T> + Sub<Output = T>>(&self, g: &[T], idx: usize) -> T {
        let mut res = T::default();
        for i in self.fold(idx, vec![1i32], |v, p, _| { for i in 0..v.len() { v.push(-v[i] * p as i32); } }) {
            // f(idx) = sum_{idx%i = 0} μ(i) g(idx/i)
            if 0 <= i { res = res + g[idx/i as usize]; } else { res = res - g[idx/-i as usize]; }
        }
        res
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
    
    pub fn mul_mobius_point<T: Default + Copy + Add<Output = T> + Sub<Output = T>>(&self, g: &[T], idx: usize) -> T {
        let mut res = T::default();
        let gl = g.len() - 1;
        let t = self.mobius(gl/idx);
        for (i, m) in t.into_iter().enumerate() {
            if m == 1 { res = res + g[i*idx]; } else if m == -1 { res = res - g[i*idx]; }
        }
        res
    }
}



// /// `n` が素数かを判定する。
// /// 
// /// 適切な `al` を選ぶことで決定的アルゴリズムになるらしい。
// pub fn millar_rabin(n: u64) -> bool {
//     assert!(n != 0);
//     if n == 2 { return true; }
//     if n == 1 || n%2 == 0 { return false; }
    
//     let (mut s, mut d) = (0, n);
//     while d%2 == 0 { s += 1; d /= 2; }
//     let mnt = Montgomery::new(n);
//     let al: &[u64] = if n < 4759123141 { &[2, 7, 61] } else { &[2, 325, 9375, 28178, 450775, 9780504, 1795265022] };
    
//     'a: for &a in al {
//         // if n <= a { return true; }
//         let mut x = mnt.pow(mnt.prod_r(a), d);
//         if x == 1 { continue; }
//         for _ in 0..s {
//             if x == n-1 { continue 'a; }
//             x = mnt.prod_rinv(x*x);
//         }
//         return false;
//     }
//     true
// }
