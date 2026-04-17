pub trait IterUtil {
    type T;
    fn into_vec(self) -> Vec<Self::T>;
}

impl<T, U: Iterator<Item = T>> IterUtil for U {
    type T = T;
    fn into_vec(self) -> Vec<Self::T> { self.collect() }
}
