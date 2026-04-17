pub struct BinaryTrie<const B: usize> {
    /// `dat[2idx + 0..2]`: 遷移先
    pub dat: Vec<usize>,
    cnt: Vec<i64>,
}

impl<const B: usize> BinaryTrie<B> {
    pub fn new() -> Self {
        Self { dat: vec![!0, !0], cnt: vec![0] }
    }
    
    pub fn len(&self) -> usize { self.cnt.len() }
    
    pub fn add(&mut self, mut x: usize, n: i64, xor: usize) {
        assert!(x < 1<<B);
        x ^= xor;
        let mut p = 0;
        for i in (0..B).rev() {
            self.cnt[p] += n;
            let c = 2*p + (x>>i & 1);
            if self.dat[c] == !0 {
                self.dat[c] = self.len();
                self.dat.extend([!0, !0]);
                self.cnt.push(0);
            }
            p = self.dat[c];
        }
        self.cnt[p] += n;
    }
    
    pub fn count(&self, mut x: usize, xor: usize) -> i64 {
        x ^= xor;
        let mut p = 0;
        for i in (0..B).rev() {
            let c = 2*p + (x>>i & 1);
            if self.dat[c] == !0 { return 0; }
            p = self.dat[c];
        }
        self.cnt[p]
    }
    
    /// `sum(cnt[0..r]) <= n` なる最大の `r` を返す。
    pub fn lower_bound(&self, mut n: i64, xor: usize) -> usize {
        if self.cnt[0] <= n { return 1<<B; }
        let mut res = 0;
        let mut p = 0;
        for i in (0..B).rev() {
            let mut c = 2*p + (xor>>i & 1);
            let ln = *self.cnt.get(self.dat[c]).unwrap_or(&0);
            if ln <= n { res |= 1<<i; n -= ln; c ^= 1; }
            p = self.dat[c];
        }
        res
    }
    
    /// `sum(cnt[0..x])` を返す。
    pub fn sum(&self, mut x: usize, xor: usize) -> i64 {
        assert!(x <= 1<<B);
        if x == 1<<B { return self.cnt[0]; }
        let mut res = 0;
        let mut p = 0;
        for i in (0..B).rev() {
            let mut c = 2*p + (xor>>i & 1);
            if 1<<i <= x { res += self.cnt.get(self.dat[c]).unwrap_or(&0); x ^= 1<<i; c ^= 1; }
            p = self.dat[c];
            if p == !0 { break; }
        }
        res
    }
}
