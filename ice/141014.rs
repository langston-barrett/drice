#![feature(min_generic_const_args)]

trait Abc {}

trait A {
    const VALUE: usize;
}

impl<T: Abc> A for T {
    const VALUE: usize = 0;
}

trait S<const K: usize> {}

trait Handler<T>
where
    // T: A,
    (): S<{ <T as A>::VALUE }>,
{
}