#![feature(non_lifetime_binders, generic_const_exprs)]
type Array<T> = [i32; { let _: T = todo!(); 0 }];

fn foo() -> usize
where
    for<T> Array<T>:,
{ 42 }

fn main() {}