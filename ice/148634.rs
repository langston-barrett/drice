#![feature(repr_simd)]

use std::arch::asm;

#[repr(simd)]
enum Es {}
static CLs: Es;

pub fn main() {
    let mut b = 4i32;
    unsafe {
        asm!("{}", out(reg) CLs);
    }
}