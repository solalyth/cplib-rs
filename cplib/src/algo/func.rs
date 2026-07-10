pub fn next_permutation<T: Ord>(v: &mut [T]) -> bool {
    let Some(i) = v.windows(2).rposition(|w| w[0] < w[1]) else { return false; };
    let j = v.iter().rposition(|e| e > &v[i]).unwrap();
    v.swap(i, j);
    v[i+1..].reverse();
    true
}

pub fn prev_permutation<T: Ord>(v: &mut [T]) -> bool {
    let Some(i) = v.windows(2).rposition(|w| w[0] > w[1]) else { return false; };
    let j = v.iter().rposition(|e| e < &v[i]).unwrap();
    v.swap(i, j);
    v[i+1..].reverse();
    true
}

pub fn run_length<T: Eq>(iter: impl IntoIterator<Item = T>) -> Vec<(T, usize)> {
let mut res = vec![];
    for t in iter {
        let Some(l) = res.last_mut() else { res.push((t, 1)); continue; };
        if t == l.0 { l.1 += 1; } else { res.push((t, 1)); }
    }
    res
}


/// `prefix[i] = fold(0..i)`
pub fn prefix_fold<T, U>(iter: impl IntoIterator<Item = U>, init: T, mut f: impl FnMut(&T, U) -> T) -> Vec<T> {
    let mut res = vec![init];
    for u in iter { res.push(f(res.last().unwrap(), u)); }
    res
}


/// `suffix[i] = fold(i..)`
pub fn suffix_fold<T, U>(iter_rev: impl IntoIterator<Item = U>, init: T, mut f: impl FnMut(&T, U) -> T) -> Vec<T> {
    let mut res = vec![init];
    for u in iter_rev { res.push(f(res.last().unwrap(), u)); }
    res.reverse();
    res
}



#[macro_export]
/// `inf` に対して `2*inf-2` が表現可能か気を付ける。
macro_rules! binary_search {
    ($low:expr, $high:expr) => {
        if 1<$high.wrapping_sub($low) { Some($high.wrapping_add($low)/2) } else { None }
    }
}
