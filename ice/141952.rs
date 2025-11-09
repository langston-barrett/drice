#![allow(incomplete_features)]
#![feature(ergonomic_clones)]

use std::clone::UseCloned;

impl UseCloned for Option<(S, S, S)> {}

struct S;

fn main() {
    macro_rules! m {
        ($p:pat = $s:expr) => {
            match $s {
                Some($p) => {}
                _ => {}
            }
        };
    }
    let mut tup1: Option<(S, S, S)> = None;
    let mut _closure = use || {
        m!((_, _, ref mut _borrow) = tup1);
    };
}