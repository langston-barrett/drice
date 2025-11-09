#![feature(type_alias_impl_trait)]

pub type Opaque = impl std::future::Future;

trait Foo<const N: Opaque> {
    fn do_x(&self) -> [Opaque; N];
}

struct Bar;

impl Foo<3> for Bar {
    fn do_x(&self) -> [u8; 3] {
        [0u8; 3]
    }
}

fn main() {}