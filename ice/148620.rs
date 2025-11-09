#![feature(associated_const_equality)]

trait Trait<'a> {
    const K: &'a ();
}

fn main() -> Trait<'r, K = { &() }> {
    let x = Ok(42);
    if true {
        x?
    }

    Ok(())
}