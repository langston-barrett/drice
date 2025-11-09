#[allow(dead_code)]
use std::arch::global_asm;

mod a {
    pub static X: isize = 3;
}

global_asm! {
    "{}",
    sym a::X::<{}>,
}

fn main() {}