//! 標準出力を楽にするライブラリ

#![allow(static_mut_refs, non_camel_case_types)]

use std::{mem::replace, ops::{Not, Shl}, fmt::Write};



static mut BUFFER: Buffer = Buffer { buf: String::new(), endp: false, prev: Previous::LineHead };

/// # Fields
/// 
/// `endp`: `out << end;` で出力するならば `true`
pub struct Buffer {
    buf: String,
    endp: bool,
    prev: Previous,
}

impl Buffer {
    const LEN: usize = 16*1024*1024; // 16MiB, 4e6 chars
    
    fn print(&mut self) {
        if replace(&mut self.prev, Previous::LineHead) == Previous::LineHead { self.buf.pop(); }
        if crate::cplib::SUBMISSION {
            println!("{}", self.buf);
        } else {
            eprint!("\x1b[32m");
            if self.buf.is_empty() {
                println!(">> (empty)");
            } else {
                for s in self.buf.split('\n') {
                    eprint!(">> ");
                    println!("{s}");
                }
            }
            eprint!("\x1b[0m");
        }
        self.buf.clear();
    }
    
    /// フラグと `sp`
    fn space(&mut self, sp: bool) {
        let prev = replace(&mut self.prev, if sp {Previous::Space} else {Previous::NoSpace});
        if (sp || prev == Previous::Space) && prev != Previous::LineHead { self.buf.push(' '); }
    }
}


#[derive(Clone, Copy)]
pub struct out;
pub struct out_usp;
pub struct end;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Previous {
    Space,
    NoSpace,
    LineHead,
}



impl out {
    pub fn init(endp: bool) {
        unsafe {
            BUFFER.buf.reserve(Buffer::LEN);
            BUFFER.endp = endp;
        }
    }
    pub fn print() { unsafe { BUFFER.print(); } }
    
    pub fn space() { unsafe { if BUFFER.prev == Previous::NoSpace { BUFFER.prev = Previous::Space; } } }
    fn push<T: Primitive>(v: &T) {
        unsafe {
            BUFFER.space(true);
            v.fmt(&mut BUFFER.buf);
        }
    }
}

impl out_usp {
    fn push<T: Primitive>(v: &T) {
        unsafe {
            BUFFER.space(false);
            v.fmt(&mut BUFFER.buf);
        }
    }
}

impl Not for out {
    type Output = out_usp;
    fn not(self) -> Self::Output { out_usp }
}



/// implement `Shl<Primitive>, Shl<end>` for `end, end_usp`
macro_rules! impl_outs {
    ($($t:ty),+) => { $(
        impl<T: Primitive> Shl<T> for $t {
            type Output = Self;
            fn shl(self, rhs: T) -> Self::Output {
                Self::push(&rhs); self
            }
        }
        impl Shl<end> for $t {
            type Output = Self;
            fn shl(self, _: end) -> Self::Output {
                unsafe {
                    if BUFFER.endp {
                        BUFFER.print();
                    } else {
                        BUFFER.buf += "\n";
                        BUFFER.prev = Previous::LineHead;
                    }
                }
                self
            }
        }
    )+ };
}
impl_outs!(out, out_usp);






macro_rules! impl_for_slices {
    ($($t:ty),+) => { $(impl_for_slices!($t; out, out_usp);)+ };
    ($t:ty; $($u:ty),+) => { $(
        impl<T: Primitive> Shl<$t> for $u {
            type Output = Self;
            fn shl(self, rhs: $t) -> Self::Output { for v in rhs { Self::push(v); } self }
        }
    )+}
}
impl_for_slices!(&[T], &Vec<T>);









trait Primitive {
    fn fmt(&self, buf: &mut String);
}

macro_rules! impl_primitive {
    ($($t:ty),+) => { $(
        impl Primitive for $t {
            fn fmt(&self, buf: &mut String) {
                write!(buf, "{self}").ok();
            }
        }
    )+ }
}
impl_primitive!(char, u32, u64, u128, usize, i32, i64, i128, f32, f64, &str, &String, String);

impl Primitive for u8 {
    fn fmt(&self, buf: &mut String) {
        buf.push(*self as char);
    }
}

impl Primitive for bool {
    fn fmt(&self, buf: &mut String) {
        *buf += if *self { "Yes" } else { "No" };
    }
}
