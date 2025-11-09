// 148164

trait Trait1 {}
auto trait Trait2 {}

fn foo(_: Box<dyn Trait1 + !Trait2>) {}

fn main() {}
