pub use {
    proconio::{
        input, input_interactive,
        marker::{Bytes as bytes, Chars as chars, Usize1 as u1}
    },
    itertools::{Itertools, iproduct, izip},
    superslice::Ext,
    num_integer::{gcd, lcm, Roots},
    ac_library::{self, ModInt998244353 as Mint},
    
    rand
};

#[macro_export]
macro_rules! input_one {
    () => { { input! { x: usize } x } };
}
