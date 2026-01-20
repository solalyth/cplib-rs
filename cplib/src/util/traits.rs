//! # Implemented trait list
//! 
//! + [`Grid`]
//! + [`CharUtil`]
//! + [`MapInit`]


/// trait for `[usize; 2]`, `[i64; 2]`
pub trait Grid: Copy + Default {
    // 順番はもう諦めることにしました
    const AROUND: [[i64; 2]; 8] = [[0, -1], [0, 1], [-1, 0], [1, 0], [-1, -1], [-1, 1], [1, -1], [1, 1]];
    fn add(self, rhs: [i64; 2]) -> Self;
    fn add_char(self, c: char, n: i64) -> Self {
        let mut d = Self::AROUND[match c { 'L' => 2, 'R' => 3, 'U' => 1, 'D' => 0, _ => unreachable!() }];
        d[0] *= n; d[1] *= n;
        self.add(d)
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
    
    fn us(self) -> usize;
    fn parse_lower(self) -> usize;
    fn parse_upper(self) -> usize;
    fn parse_digit(self) -> usize;
    
    fn flip(self) -> Self;
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
    
    fn us(self) -> usize { (self as u8 - 65 & 223) as usize }
    fn parse_lower(self) -> usize { debug_assert!('a' <= self && self <= 'z'); self as usize - 97 }
    fn parse_upper(self) -> usize { debug_assert!('A' <= self && self <= 'Z'); self as usize - 65 }
    fn parse_digit(self) -> usize { debug_assert!('0' <= self && self <= '9'); self as usize - 48 }
    
    fn flip(self) -> Self { (self as u8 ^ 32) as char }
}



use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub trait MapInit {
    type K;
    type V;
    fn init(&mut self, key: Self::K, init: Self::V) -> &mut Self::V;
    fn init_with(&mut self, key: Self::K, init: impl FnOnce() -> Self::V) -> &mut Self::V;
}

impl<K: Eq + Hash, V> MapInit for HashMap<K, V> {
    type K = K;
    type V = V;
    fn init(&mut self, key: K, init: V) -> &mut V {
        self.entry(key).or_insert(init)
    }
    fn init_with(&mut self, key: K, init: impl FnOnce() -> Self::V) -> &mut V {
        self.entry(key).or_insert_with(init)
    }
}

impl<K: Ord, V> MapInit for BTreeMap<K, V> {
    type K = K;
    type V = V;
    fn init(&mut self, key: K, init: V) -> &mut V {
        self.entry(key).or_insert(init)
    }
    fn init_with(&mut self, key: Self::K, init: impl FnOnce() -> Self::V) -> &mut Self::V {
        self.entry(key).or_insert_with(init)
    }
}



pub trait VecSplit {
    type Output;
    fn split(self) -> Self::Output;
}

impl<T0, T1> VecSplit for Vec<(T0, T1)> {
    type Output = (Vec<T0>, Vec<T1>);
    fn split(self) -> Self::Output {
        let mut res = (vec![], vec![]);
        for e in self { res.0.push(e.0); res.1.push(e.1); }
        res
    }
}

impl<T0, T1, T2> VecSplit for Vec<(T0, T1, T2)> {
    type Output = (Vec<T0>, Vec<T1>, Vec<T2>);
    fn split(self) -> Self::Output {
        let mut res = (vec![], vec![], vec![]);
        for e in self { res.0.push(e.0); res.1.push(e.1); res.2.push(e.2); }
        res
    }
}

// impl<T0, T1, T2, T3> VecSplit for Vec<(T0, T1, T2, T3)> {
//     type Output = (Vec<T0>, Vec<T1>, Vec<T2>, Vec<T3>);
//     fn split(self) -> Self::Output {
//         let mut res = (vec![], vec![], vec![], vec![]);
//         for e in self { res.0.push(e.0); res.1.push(e.1); res.2.push(e.2); res.3.push(e.3); }
//         res
//     }
// }
