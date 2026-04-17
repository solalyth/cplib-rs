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
