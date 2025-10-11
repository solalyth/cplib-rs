pub const M: u64 = 998244353;
pub const R: u64 = (1<<32) % M;
pub const R2: u64 = (R*R) % M;
const P: u32 = 998244351;

/// `x in [0, MR)` について `x * inv(R) mod M` を返す。
pub fn prod_rinv(x: u64) -> u64 {
    debug_assert!(x < (M << 32));
    let a = P.wrapping_mul(x as u32) as u64;
    let b = (x + a*M) >> 32;
    if M <= b { b-M } else { b }
}

/// `x <= u32::MAX` について `xR mod M` を返す。
pub fn prod_r(x: u64) -> u64 {
    debug_assert!(x < 1<<32);
    prod_rinv(x * R2)
}

pub fn add(l: u64, r: u64) -> u64 {
    debug_assert!(l.max(r) < M);
    if M <= l+r { l+r-M } else { l+r }
}

pub fn sub(l: u64, r: u64) -> u64 {
    debug_assert!(l.max(r) < M);
    if l < r { l+M-r } else { l-r }
}

/// `x < MR` について `x mod M` を返す。
pub fn rem(x: u64) -> u64 {
    debug_assert!(x < M<<32);
    prod_r(prod_rinv(x))
}
