#![feature(type_alias_impl_trait)]

type Foo = impl Send;

trait Trait {
    const CONST: u32;
}

impl Trait for Foo {
    const CONST: u32 = 0;
}

mod qself_in_const {
    #[repr(u32)]
    enum QSelfInConst {
        Variant = <::Foo as crate::Trait>::CONST,
    }
}

fn main() {}