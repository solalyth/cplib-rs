/// 区間 `0..r` を `(L, B)` のペアの列に変換する。下位 `B` bit 以外は `L` と一致する。`B == 0` の場合に注意。
pub fn decomp_prefix(r: usize) -> Vec<(usize, usize)> {
    let (mut res, mut l) = (vec![], 0);
    for b in (0..64).rev() {
        if r>>b&1 == 1 { res.push((l, b as usize)); l += 1<<b; }
    }
    res
}

/// 区間 `l..r` を `(L, B)` のペアの列に変換する。下位 `B` bit 以外は `L` と一致する。`B == 0` の場合に注意。
pub fn decomp_range(mut l: usize, r: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    if l >= r { return res; }
    for b in 0..63 {
        if !(1<<b <= r-l) { break; }
        if l>>b & 1 == 1 { res.push((l, b as usize)); l += 1<<b; }
    }
    for b in (0..63).rev() {
        if 1<<b <= r-l { res.push((l, b as usize)); l += 1<<b; }
        if l == r { break; }
    }
    res
}
