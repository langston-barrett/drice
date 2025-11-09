#![feature(unboxed_closures)]

use std::future::Future;

trait Foo {
    fn foo() -> impl Sized
    where
        for<'a> <dyn Foo as FnOnce<(&'a mut i32,)>>::Output: Future<Output = ()> + 'a,
    {
    }
}

fn main() {}