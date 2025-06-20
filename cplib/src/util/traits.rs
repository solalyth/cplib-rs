pub trait GridIndex: Sized + Copy + Default {
    type T: Copy;
    /// 上、右上、右、右下、下、左下、左、左上
    const AROUND: [Self::T; 8];
    fn add(self, rhs: Self::T) -> Self;
    fn around4(self) -> [Self; 4] {
        let mut res = [Self::default(); 4];
        for i in 0..4 { res[i] = self.add(Self::AROUND[2*i]); }
        res
    }
    fn around8(self) -> [Self; 8] {
        let mut res = [Self::default(); 8];
        for i in 0..8 { res[i] = self.add(Self::AROUND[i]); }
        res
    }
    fn rotate(self, l: usize, n: i32) -> Self;
}

impl GridIndex for [usize; 2] {
    type T = [isize; 2];
    const AROUND: [Self::T; 8] = [[-1, 0], [-1, 1], [0, 1], [1, 1], [1, 0], [1, -1], [0, -1], [-1, -1]];
    fn add(self, rhs: Self::T) -> Self {
        [self[0].wrapping_add_signed(rhs[0]), self[1].wrapping_add_signed(rhs[1])]
    }
    fn rotate(self, l: usize, mut n: i32) -> Self {
        let [i, j] = self;
        n = n.div_euclid(4);
        if n == 0 { self }
        else if n == 1 { [j, l-1-i] }
        else if n == 2 { [l-1-i, l-1-j] }
        else { [l-1-j, i] }
    }
}

impl GridIndex for (usize, usize) {
    type T = (isize, isize);
    const AROUND: [Self::T; 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
    fn add(self, rhs: Self::T) -> Self {
        (self.0.wrapping_add_signed(rhs.0), self.1.wrapping_add_signed(rhs.1))
    }
    fn rotate(self, l: usize, mut n: i32) -> Self {
        let (i, j) = self;
        n = n.div_euclid(4);
        if n == 0 { self }
        else if n == 1 { (j, l-1-i) }
        else if n == 2 { (l-1-i, l-1-j) }
        else { (l-1-j, i) }
    }
}



pub trait CharUtil: Clone {
    const LOWER: [Self; 26];
    const UPPER: [Self; 26];
    
    fn lower_to_us(self) -> usize;
    fn upper_to_us(self) -> usize;
    fn parse_us(self) -> usize;
    
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
    
    fn lower_to_us(self) -> usize { debug_assert!('a' <= self && self <= 'z'); self as usize - 97 }
    fn upper_to_us(self) -> usize { debug_assert!('A' <= self && self <= 'Z'); self as usize - 65 }
    fn parse_us(self) -> usize { self as usize - 48 }
    
    fn flip(self) -> Self { (self as u8 ^ 32) as char }
    
    fn as_urdl(self) -> usize { [b'U', b'R', b'D', b'L'].iter().position(|&v| v == self as u8 & 32).unwrap() }
}
