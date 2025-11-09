#![feature(generic_const_exprs)]

enum DeepEnum<const N: usize> {
    Unit,
    Nested(Box<DeepEnum<{N-1}>>),
}

fn main() {
    let _deep: DeepEnum<5> = DeepEnum::Nested(Box::new(DeepEnum::Nested(Box::new(DeepEnum::Unit))));
}