pub use {
    proconio::{
        input, input_interactive,
        marker::{Bytes as bytes, Chars as chars, Usize1 as u1},
        source::once::OnceSource,
    },
    itertools::{Itertools, iproduct, izip},
    superslice::Ext,
    num_integer::{gcd, lcm, Roots},
    ac_library::{self, ModInt998244353 as Mint},
    
    rand
};
