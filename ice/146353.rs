#![feature(trait_alias)]

use std::mem::{MaybeUninit};
const BIG_CHAIN = MaybeUninit::uninit();

pub trait NeverSend = !Send;