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
        if t == l.0 {
            l.1 += 1;
        } else {
            res.push((t, 1));
        }
    }
    res
}


/// 隣接がマージ可能なら圧縮するやつ。`f(lhs, rhs)` について、マージ可能ならば `lhs` にマージさせて `true` を返し、不可能ならばそのまま `false` を返すこと。
pub fn coalesce_inplace<T>(v: &mut Vec<T>, f: impl Fn(&mut T, &mut T) -> bool) {
    let mut l = 0;
    for r in 1..v.len() {
        let (sl, sr) = v.split_at_mut(r);
        if !f(&mut sl[l], &mut sr[0]) {
            v.swap(l+1, r);
            l += 1;
        }
    }
    v.truncate(l+1);
}



/// `prefix[i] = fold(0..i)`
/// 
/// `suffix[i]` が欲しいときは `iter` を逆順にして渡したあと `reverse` を取ればよい。
pub fn prefix_fold<T, U>(init: T, iter: impl IntoIterator<Item = U>, mut f: impl FnMut(&T, U) -> T) -> Vec<T> {
    let mut res = vec![init];
    for u in iter { res.push(f(res.last().unwrap(), u)); }
    res
}

/// `suffix[i] = fold(i..)`
pub fn suffix_fold<T, U>(init: T, iter: impl Iterator<Item = U> + DoubleEndedIterator, mut f: impl FnMut(&T, U) -> T) -> Vec<T> {
    let mut res = vec![init];
    for u in iter.rev() { res.push(f(res.last().unwrap(), u)); }
    res.reverse();
    res
}



pub fn binary_search(low: usize, high: usize) -> Option<usize> {
    if 1 < high.wrapping_sub(low) { Some(high.wrapping_add(low)/2) } else { None }
}
