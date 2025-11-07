pub use {
    // atcoder, yukicoder
    proconio::{
        input, input_interactive,
        marker::{Chars as chars, Usize1 as usize1}
    },
    
    // atcoder only
    itertools::{Itertools, iproduct},
    superslice::Ext,
    num_integer::{gcd, lcm, Roots},
    // num_bigint::BigUint,
    ac_library::{self, ModInt998244353 as Mint},
    rand
};

#[cfg(feature = "un_contest")]
mod util;

#[cfg(feature = "un_contest")]
pub use util::*;
