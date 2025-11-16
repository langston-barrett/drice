#![feature(unsafe_binders)]
#![feature(transmutability)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

trait OpaqueTrait {}
type OpaqueType = unsafe<> impl OpaqueTrait;
trait AnotherTrait {}
impl<T: std::mem::TransmuteFrom<()>> AnotherTrait for T {}
impl AnotherTrait for OpaqueType {}

pub fn main() {}