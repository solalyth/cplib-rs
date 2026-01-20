use std::{cell::UnsafeCell, mem::MaybeUninit};

pub struct Global<T>(UnsafeCell<MaybeUninit<T>>);

impl<T> Global<T> {
    pub const fn uninit() -> Self { Self(UnsafeCell::new(MaybeUninit::uninit())) }
    pub const fn new(value: T) -> Self { Self(UnsafeCell::new(MaybeUninit::new(value))) }
    pub fn init(&self, value: T) -> &mut T { unsafe { (*self.0.get()).write(value); } self.get() }
    pub fn get(&self) -> &mut T { unsafe { (&mut *self.0.get()).assume_init_mut() } }
}

unsafe impl<T> Sync for Global<T> {}
