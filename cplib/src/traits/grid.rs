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
    
    fn rotate(self, n: usize, m: usize, t: i64) -> Self;
}

impl Grid for [usize; 2] {
    fn add(mut self, rhs: [i64; 2]) -> Self {
        for i in 0..2 {
            self[i] = self[i].wrapping_add_signed(rhs[i] as isize);
        }
        self
    }
    
    /// `h*w` グリッド内で `t` 回だけ時計回りに回転させる。
    fn rotate(self, h: usize, w: usize, t: i64) -> Self {
        let [i, j] = self;
        match t.rem_euclid(4) {
            0 => [i, j],
            1 => [j, h-1-i],
            2 => [h-1-i, w-1-j],
            3 => [w-1-j, i],
            _ => unreachable!()
        }
    }
}

impl Grid for [i64; 2] {
    fn add(mut self, rhs: [i64; 2]) -> Self {
        for i in 0..2 { self[i] += rhs[i]; }
        self
    }
    fn rotate(self, _: usize, _: usize, _: i64) -> Self { unimplemented!() }
}
