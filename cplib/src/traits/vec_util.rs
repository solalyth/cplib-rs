pub trait VecUtil {
    type T;
    fn extend_len(&mut self, len: usize, e: Self::T) where Self::T: Clone;
    /// 隣接要素がマージ可能ならマージする。
    /// 
    /// `f(l, r)` について、マージ可能ならば `l` にマージさせて `true` を返す。マージ不可能ならば `false` を返す。
    fn adj_merge(&mut self, f: impl FnMut(&mut Self::T, &mut Self::T) -> bool);
}

impl<T> VecUtil for Vec<T> {
    type T = T;
    fn extend_len(&mut self, len: usize, e: T) where T: Clone {
        if self.len() < len { self.resize(len, e); }
    }
    fn adj_merge(&mut self, mut f: impl FnMut(&mut Self::T, &mut Self::T) -> bool) {
        let mut l = 0;
        for r in 1..self.len() {
            // let sl = self.split_at_mut(r);
            // let t = f(&mut sl.0[l], &mut sl.1[0]);
            let t = unsafe {
                let [sl, sr] = self.get_disjoint_unchecked_mut([l, r]);
                f(sl, sr)
            };
            if !t {
                self.swap(l+1, r);
                l += 1;
                
            }
        }
        self.truncate(l+1);
    }
}
