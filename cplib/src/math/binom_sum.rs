use crate::math::modtable::O;

pub struct BinomSum {
    n: usize,
    l: usize,
    r: usize,
    s: usize,
}

impl BinomSum {
    pub fn new() -> Self {
        Self { n: 0, l: 0, r: 0, s: 0 }
    }
    
    pub fn get(&mut self, n: usize, l: usize, mut r: usize) -> usize {
        r = r.min(n+1);
        if l >= r { return 0; }
        let p = O.MOD();
        while self.n < n {
            self.s = p + 2*self.s + O.C(self.n, self.l.wrapping_sub(1)) - O.C(self.n, self.r.wrapping_sub(1));
            self.s %= p;
            self.n += 1;
        }
        while n < self.n {
            self.n -= 1;
            self.s = p + self.s - O.C(self.n, self.l.wrapping_sub(1)) + O.C(self.n, self.r.wrapping_sub(1));
            if self.s%2 == 1 { self.s += p; }
            self.s /= 2;
        }
        while l < self.l {
            self.l -= 1;
            self.s += O.C(self.n, self.l)%p;
        }
        while self.r < r {
            self.s += O.C(self.n, self.r);
            self.r += 1;
        }
        while self.l < l {
            self.s += p - O.C(self.n, self.l);
            self.l += 1;
        }
        while r < self.r {
            self.r -= 1;
            self.s += p - O.C(self.n, self.r);
        }
        self.s %= p;
        self.s
    }
}
