#![feature(associated_const_equality)]

trait Project {
    const SELF: [Self; 1];
}

fn take1(_: Project<SELF = {}>) {}