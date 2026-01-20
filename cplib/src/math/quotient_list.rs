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
