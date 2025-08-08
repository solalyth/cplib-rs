//! # Implemented trait list
//! 
//! + [`Grid`]
//! + [`CharUtil`]
//! + [`SaturatingPow`]
//! + [`GetOrInsert`]


/// trait for `[usize; 2]`, `[i64; 2]`
pub trait Grid: Copy + Default {
    // 順番はもう諦めることにしました
    const AROUND: [[i64; 2]; 8] = [[0, -1], [0, 1], [-1, 0], [1, 0], [-1, -1], [-1, 1], [1, -1], [1, 1]];
    fn add(self, rhs: [i64; 2]) -> Self;
    fn apply(self, c: char) -> Self {
        self.add(Self::AROUND[match c { 'L' => 0, 'R' => 1, 'U' => 2, 'D' => 3, _ => unreachable!() }])
    }
    fn around4(self) -> [Self; 4] {
        let mut res = [Default::default(); 4];
        for i in 0..4 { res[i] = self.add(Self::AROUND[i]); }
        res
    }
    fn around8(self) -> [Self; 8] {
        let mut res = [Default::default(); 8];
        for i in 0..8 { res[i] = self.add(Self::AROUND[i]); }
        res
    }
    
    fn rotate(self, n: usize, t: i64) -> Self;
}

impl Grid for [usize; 2] {
    fn add(mut self, rhs: [i64; 2]) -> Self {
        for i in 0..2 {
            self[i] = self[i].wrapping_add_signed(rhs[i] as isize);
        }
        self
    }
    
    /// `n*n` 行列内で `t` 回だけ時計回りに回転させる。
    fn rotate(self, n: usize, t: i64) -> Self {
        let [i, j] = self;
        match t.rem_euclid(4) {
            0 => [i, j],
            1 => [j, n-1-i],
            2 => [n-1-i, n-1-j],
            3 => [n-1-j, i],
            _ => unreachable!()
        }
    }
}

impl Grid for [i64; 2] {
    fn add(mut self, rhs: [i64; 2]) -> Self {
        for i in 0..2 { self[i] += rhs[i]; }
        self
    }
    fn rotate(self, _: usize, _: i64) -> Self { unimplemented!() }
}



/// |Range|Chars|
/// |---|---|
/// |`33..48`|`!"#$%&'()*+,-./`|
/// |`48..58`|`0123456789`|
/// |`58..65`|`:;<=>?@`|
/// |`65..91`|`ABCDEFGHIJKLMNOPQRSTUVWXYZ`|
/// |`91..97`|``[\]^_` ``|
/// |`97..123`|`abcdefghijklmnopqrstuvwxyz`|
/// |`123..127`|`{\|}~`|
pub trait CharUtil: Clone {
    const LOWER: [Self; 26];
    const UPPER: [Self; 26];
    const NUMBER: [Self; 10];
    
    fn parse_lower(self) -> usize;
    fn parse_upper(self) -> usize;
    fn parse_digit(self) -> usize;
    
    fn flip(self) -> Self;
    
    fn as_urdl(self) -> usize;
}

impl CharUtil for char {
    const LOWER: [char; 26] = {
        let (mut out, mut i) = (['_'; 26], 0);
        while i < 26 { out[i] = (i+97) as u8 as char; i += 1; }
        out
    };
    
    const UPPER: [char; 26] = {
        let (mut out, mut i) = (['_'; 26], 0);
        while i < 26 { out[i] = (i+65) as u8 as char; i += 1; }
        out
    };
    
    const NUMBER: [char; 10] = {
        let (mut res, mut i) = (['_'; 10], 0);
        while i < 10 { res[i] = (i+48) as u8 as char; i += 1; }
        res
    };
    
    // ('a'..='z').contains(&self)
    fn parse_lower(self) -> usize { debug_assert!('a' <= self && self <= 'z'); self as usize - 97 }
    fn parse_upper(self) -> usize { debug_assert!('A' <= self && self <= 'Z'); self as usize - 65 }
    fn parse_digit(self) -> usize { debug_assert!('0' <= self && self <= '9'); self as usize - 48 }
    
    fn flip(self) -> Self { (self as u8 ^ 32) as char }
    
    fn as_urdl(self) -> usize { [b'U', b'R', b'D', b'L'].iter().position(|&v| v == self as u8).unwrap() }
}



pub trait SaturatingPow {
    /// ただし `0^0 = 1` とする。
    fn saturating_pow(self, exp: usize) -> Self;
}

macro_rules! impl_saturating_pow {
    ($($t:ty),+) => { $(
        impl SaturatingPow for $t {
            fn saturating_pow(mut self, mut exp: usize) -> Self {
                let mut res = 1 as $t;
                while exp != 0 {
                    if exp%2 == 1 {
                        res = res.saturating_mul(self);
                    }
                    self = self.saturating_mul(self);
                    exp /= 2;
                }
                res
            }
        }
    )+ };
}

impl_saturating_pow!(usize, i64);



use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub trait GetOrInsert {
    type K;
    type V;
    fn get_or_insert(&mut self, key: Self::K, init: Self::V) -> &mut Self::V;
}

impl<K: Eq + Hash, V> GetOrInsert for HashMap<K, V> {
    type K = K;
    type V = V;
    fn get_or_insert(&mut self, key: K, init: V) -> &mut V {
        self.entry(key).or_insert(init)
    }
}

impl<K: Ord, V> GetOrInsert for BTreeMap<K, V> {
    type K = K;
    type V = V;
    fn get_or_insert(&mut self, key: K, init: V) -> &mut V {
        self.entry(key).or_insert(init)
    }
}



// pub trait UsizeUtil: Copy {
//     /// `(0..n).sum() == n*(n-1)/2`
//     fn linear_sum(self) -> Self;
//     fn inv_linear_sum(self) -> Self;
    
//     fn sqrt(self) -> Self;
// }

// impl UsizeUtil for usize {
//     fn linear_sum(self) -> Self {
//         self*self.wrapping_sub(1)/2
//     }
//     fn inv_linear_sum(self) -> Self {
//         todo!()
//     }
    
//     fn sqrt(self) -> Self {
        
//     }
// }
