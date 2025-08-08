//! 高速出力

#![allow(static_mut_refs)]

use std::{mem::replace, ops::{Not, Shl}, fmt::Write};

static mut BUFFER: Buffer = Buffer { buf: String::new(), endp: false, prev: Previous::LineHead };
#[allow(non_upper_case_globals)]
pub static out: Output = Output;


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
        if self.buf.is_empty() {
            if !crate::cplib::SUBMISSION { println!("\x1b[32m>> (empty)\x1b[0m"); }
            return;
        }
        if crate::cplib::SUBMISSION {
            println!("{}", self.buf);
        } else {
            eprint!("\x1b[32m");
            for s in self.buf.split('\n') {
                eprint!(">> ");
                println!("{s}");
            }
            eprint!("\x1b[0m");
        }
        self.buf.clear();
    }
    
    fn space(&mut self, sp: bool) {
        let prev = replace(&mut self.prev, if sp {Previous::Space} else {Previous::NoSpace});
        if (sp || prev == Previous::Space) && prev != Previous::LineHead { self.buf.push(' '); }
    }
}


#[derive(Clone, Copy)]
pub struct Output<const SP: bool = true>;

impl Output {
    pub fn init(endp: bool) {
        unsafe {
            BUFFER.buf.reserve(Buffer::LEN);
            BUFFER.endp = endp;
        }
    }
    pub fn print() { unsafe { BUFFER.print(); } }
}

impl<const SP: bool> Output<SP> {
    pub fn space() { unsafe { if BUFFER.prev == Previous::NoSpace { BUFFER.prev = Previous::Space; } } }
    fn push<T: Primitive>(v: &T) {
        unsafe {
            BUFFER.space(SP);
            v.fmt(&mut BUFFER.buf);
        }
    }
}

impl Not for Output {
    type Output = Output<false>;
    fn not(self) -> Self::Output { Output }
}

impl<const SP: bool, T: Primitive> Shl<T> for Output<SP> {
    type Output = Self;
    fn shl(self, rhs: T) -> Self::Output { Self::push(&rhs); self }
}

macro_rules! impl_for_slices {
    ($t:ty) => {
        impl<const SP: bool, T: Primitive> Shl<$t> for Output<SP> {
            type Output = Self;
            fn shl(self, rhs: $t) -> Self::Output { for v in rhs { Self::push(v); } self }
        }
    };
    ($($t:ty),+) => { $(impl_for_slices!($t);)+ }
}
impl_for_slices!(&[T], &Vec<T>);


impl<const SP: bool> Shl<end> for Output<SP> {
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




#[allow(non_camel_case_types)]
pub struct end;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Previous {
    Space,
    NoSpace,
    LineHead,
}




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
impl_primitive!(char, u32, u64, u128, usize, i64, i128, f32, f64, &str, &String);

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
