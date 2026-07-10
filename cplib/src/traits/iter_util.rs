pub trait IterUtil {
    type T;
    fn into_vec(self) -> Vec<Self::T>;
    fn count_if(self, f: impl FnMut(Self::T) -> bool) -> usize;
}

impl<T, U: Iterator<Item = T>> IterUtil for U {
    type T = T;
    fn into_vec(self) -> Vec<Self::T> { self.collect() }
    fn count_if(self, mut f: impl FnMut(T) -> bool) -> usize {
        let mut t = 0;
        for x in self { if f(x) { t += 1; } }
        t
    }
}
