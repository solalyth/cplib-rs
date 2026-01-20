use std::ops::{Add, Div, Mul, Sub};

/// 列の Hadamard Transform を計算する。`H(HX * HY) / 2^len` が xor convolution と一致する。
/// 
/// 和について分配法則 `H(X+Y) = HX + HY` が成り立つ。また、`h([x, 0, ...]) = [x, x, ...]` である。
pub fn hadamard<T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + From<u64> + Copy>(mut v: Vec<T>, len: u64) -> Vec<T> {
    assert!(v.len() == 1<<len);
    for d in 0..len {
        let w = 1<<d;
        for i in (0..1<<len).step_by(2*w) {
            for j in i..i+w {
                (v[j], v[j+w]) = (v[j]+v[j+w], v[j]-v[j+w]);
            }
        }
    }
    v
}
