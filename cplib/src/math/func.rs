pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 { (a, b) = (b, a%b); }
    a
}

pub fn lcm(a: usize, b: usize) -> Option<usize> {
    (a/gcd(a, b)).checked_mul(b)
}

/// `ax + by = gcd(a,b)` を満たす `(x, y, gcd(a, b))` を返す。ただし `0 <= gcd(a,b)`, `max(|x|, |y|) <= max(|a|, |b|)` を満たす。
pub fn extended_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    if (a, b) == (0, 0) { return (0, 0, 0); }
    let mut st = vec![];
    while b != 0 { st.push(a/b); (a, b) = (b, a%b); }
    let (mut x, mut y) = (a.signum(), 0);
    for z in st.into_iter().rev() { (x, y) = (y, x - z*y); }
    (x, y, a.abs())
}

pub fn modinv(x: usize, m: usize) -> Option<usize> {
    let (x, _, g) = extended_gcd(x as i64, m as i64);
    if g == 1 { Some(x.rem_euclid(m as i64) as usize) } else { None }
}

// pub fn crt((a1, m1): (usize, usize), (a2, m2): (usize, usize)) -> Option<(usize, usize)> {
//     let g = gcd(m1, m2);
//     let (a1, a2) = ((a1%g) as i64, (a2%g) as i64);
//     if (a2-a1)%g as i64 != 0 { return None; }
//     let a1inv = modinv(m1/g, m2/g)?;
    
//     todo!()
// }



/// `x^n mod m` を計算する。`0^0 == 1` とする。
pub fn mpow(mut x: usize, mut n: usize, m: usize) -> usize {
    x %= m;
    if n == 0 { return if x == 0 {0} else {1}; }
    let mut res = 1;
    while n != 0 {
        if n&1 == 1 { res = res*x % m; }
        x = x*x % m;
        n >>= 1;
    }
    res
}








/// floor 商列挙 `(l, r, x)`: `for i in l..r, floor(n/i) == x`
/// 
/// # Memo
/// 
/// `floor(n/i) == x` <==> `floor(n/(x+1)) < i <= floor(n/x)`
/// 
/// 区間の形に気をつけると、
/// 
/// ```no_run
/// let b = N.isqrt();
/// for i in 1..=N/b {
///     // x = N/i
/// }
/// for x in (1..b).rev() {
///     let xl = N/(x+1);
///     let xr = N/x;
///     // for i in (xl, xr], N/i == x
/// }
/// ```
/// 
/// のようにシンプルに書ける。
/// 
/// `N <= x(x+1)` ならば `floor(N/i) = x` となる `i` が高々 1 つであることが示せる。上の実装だと `B = sqrt(N)+1` とすればよい。
pub fn quotient_floor(n: usize) -> Vec<(usize, usize, usize)> {
    let mut res = vec![];
    let b = (n as f64).sqrt() as usize + 1;
    for i in 1..=n/b {
        res.push((i, i+1, n/i));
    }
    for x in (1..b).rev() {
        let l = n/(x+1);
        let r = n/x;
        if l != r {
            res.push((l+1, r+1, x));
        }
    }
    // debug_assert!(res.windows(2).all(|w| w[0].1 == w[1].0 && w[0].2 > w[1].2));
    res
}

/// ceil 商列挙 `(l, r, x)`: `for i in l..r, ceil(n/i) == x`
/// 
/// # Memo
/// 
/// `ceil(n/i) == x` <==> `ceil(n/x) <= i < ceil(n/(x-1))`
/// 
/// `N <= (x-1)x` ならば `ceil(N/i) = x` となる `i` が高々 1 つであることが示せる。
pub fn quotient_ceil(n: usize) -> Vec<(usize, usize, usize)> {
    let mut res = vec![];
    let b = (n as f64).sqrt() as usize+2;
    for i in 1..(n+b-1)/b {
        res.push((i, i+1, (n+i-1)/i));
    }
    for x in (2..b).rev() {
        let l = (n+x-1)/x;
        let r = (n+x-2)/(x-1);
        if l != r { res.push((l, r, x)); }
    }
    res.push((n, n+1, 1));
    // debug_assert!(res.windows(2).all(|w| w[0].1 == w[1].0 && w[0].2 > w[1].2));
    res
}



/// 和が `s` である、長さ `n` の非負整数列を辞書順に返す。`res.len() == (s+n-1)! / (s! * (n-1)!)`
pub fn partitions(n: usize, s: usize) -> Option<Vec<Vec<usize>>> {
    if n == 0 && s != 0 { return None; }
    if s == 0 { return Some(vec![vec![0; n]]); }
    
    let mut cur = vec![0; n];
    cur[n-1] = s;
    let (mut res, mut t) = (vec![cur.clone()], n-1);
    
    while t != 0 {
        cur[t-1] += 1;
        cur[n-1] = std::mem::take(&mut cur[t])-1;
        if cur[n-1] == 0 { t -= 1; } else { t = n-1; }
        res.push(cur.clone());
    }
    
    Some(res)
}




pub fn into_ary(mut n: usize, base: usize) -> Vec<usize> {
    if n == 0 { return vec![]; }
    let mut res = vec![];
    while n != 0 {
        res.push(n%base); n /= base;
    }
    res
}

pub fn from_ary(d: &[usize], base: usize) -> usize {
    let mut res = 0;
    for &d in d { res = res*base + d; }
    res
}
