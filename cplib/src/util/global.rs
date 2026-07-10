use std::{cell::UnsafeCell, fmt::Debug, mem::MaybeUninit, ops::Deref};

pub struct Global<T>(UnsafeCell<MaybeUninit<T>>);

impl<T> Global<T> {
    pub const fn new() -> Self { Self(UnsafeCell::new(MaybeUninit::uninit())) }
    pub fn set_global(&self, value: T) { unsafe { (*self.0.get()).write(value); } }
    pub fn get_global(&self) -> &T { unsafe { (&mut *self.0.get()).assume_init_ref() } }
    pub fn get_mut_global(&self) -> &mut T { unsafe { (&mut *self.0.get()).assume_init_mut() } }
}

impl<T> Deref for Global<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { self.get_global() }
}

impl<T: Debug> Debug for Global<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.get_global())
    }
}

unsafe impl<T> Sync for Global<T> {}





#[macro_export]
/// `gl!(var: T)`: グローバル変数定義
/// `gl!(var = val)`: グローバル変数初期化
/// `gl!(var) -> &mut T` グローバル変数参照
macro_rules! gl {
    ($var:ident: $t:ty) => {
        static $var: crate::util::global::Global<$t> = crate::util::global::Global::new();
    };
    ($var:ident = $val:expr) => {
        $var.set_global($val);
    };
    ($var:ident) => {
        $var.get_mut_global()
    };
    (& $var:ident) => {
        $var.get_global()
    }
}
