#![feature(associated_const_equality)]

trait B<C> {}
trait D<C, E>: B<C> + B<E> {
    fn f(&self) {}
}

trait Project {
    const SELF: dyn D<Self, Self>;
}

fn take1(_: Project<SELF = { loop {} }>) {}

fn main() {}