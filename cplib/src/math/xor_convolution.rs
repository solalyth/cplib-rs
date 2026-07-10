use std::ops::{Add, Mul, Sub};

/// 列の Hadamard 変換を計算する。逆変換は `h(X) / 2^log` である。
/// 
/// 和の分配法則 `h(X+Y) = hX + hY` が成り立つ。`hX * hY = h(xor convolution of X, Y)` が成り立つ。`h([x, 0, ...]) = [x, x, ...]` である。
/// 
/// # Panics
/// 
/// - `v.len` が二べきでないとき
pub fn hadamard<T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy>(mut v: Vec<T>) -> Vec<T> {
    let log = v.len().trailing_zeros();
    assert!(v.len() == 1<<log);
    for d in 0..log {
        let w = 1<<d;
        for i in (0..1<<log).step_by(2*w) {
            for j in i..i+w {
                (v[j], v[j+w]) = (v[j]+v[j+w], v[j]-v[j+w]);
            }
        }
    }
    v
}
