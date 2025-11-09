#![feature(type_alias_impl_trait)]

type Foo<V> = impl std::fmt::Debug;

trait Identity<Q> {
    type T;
}

trait Holds {
    type Q;
}

struct S;
struct X(S);

struct XHelper;

impl Holds for X {
    type Q = XHelper;
}

impl<Q> Clone for Foo<<S as Identity<Q>>::T>
where
    <S as Identity<Q>>::T: Clone,
    X: Holds<Q = Q>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

fn main() {}