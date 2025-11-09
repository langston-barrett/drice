#![feature(impl_trait_in_assoc_type)]

trait Foo {
    type Assoc<'a, 'b>;
    fn bar<'a, 'b>(_: &'a ()) -> Self::Assoc<'a, 'b>;
}

impl Foo for () {
    type Assoc<'a, 'b> = Box<dyn for<'c, 'd> Fn(&'c (), &'d ()) -> Self::Assoc<'c, 'd>>;
    fn bar<'a, 'b>(x: &'a ()) -> Self::Assoc<'a, 'b> {
        let closure = |x: &'a ()| -> Self::Assoc<'a, 'b> { x };
        closure
    }
}

fn main() {}