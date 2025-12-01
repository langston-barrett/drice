#![allow(dead_code)]
mod check;
mod ice;
mod rustc;

pub use check::IceStatus;
pub use check::analyze_ice;
pub use check::code_uses_internal_features;
