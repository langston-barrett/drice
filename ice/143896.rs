pub trait TraitA<'a> {
    const K: u8 = 0;
}
pub trait TraitB<T> {}

impl<T> TraitA<T> for () {}
impl<T> TraitB<T> for () where (): TraitA<T, K = 0> {}
