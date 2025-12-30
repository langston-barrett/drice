#![feature(min_generic_const_args)]
#![feature(generic_const_exprs)]
struct Both<const is_123: u32 = 3, T> {
    a: A<{ B::<1>::M }>,
}
