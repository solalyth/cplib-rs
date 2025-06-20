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

pub fn run_length<T: Eq>(iter: impl Iterator<Item = T>) -> Vec<(T, usize)> {
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
