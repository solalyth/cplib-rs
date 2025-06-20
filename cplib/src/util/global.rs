use std::{cell::UnsafeCell, mem::MaybeUninit, ops::{Deref, DerefMut}};

pub struct Global<T>(UnsafeCell<MaybeUninit<T>>);

impl<T> Global<T> {
    pub const fn uninit() -> Self { Self(UnsafeCell::new(MaybeUninit::uninit())) }
    pub fn init(&self, value: T) { unsafe { (*self.0.get()).write(value); } }
}

unsafe impl<T> Sync for Global<T> {}
impl<T> Deref for Global<T> { type Target = T; fn deref(&self) -> &Self::Target { unsafe { (*self.0.get()).assume_init_ref() } } }
impl<T> DerefMut for Global<T> { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { (*self.0.get()).assume_init_mut() } } }
