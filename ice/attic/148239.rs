#![feature(explicit_tail_calls)]

#[inline(never)]
fn op_dummy(_param: &Box<u8>) -> [u8; 1] {
    [1; 1]
}

#[inline(never)]
fn dispatch(param: &Box<u8>) -> [u8; 1] {
    become op_dummy(param)
}

pub fn main() {
    let param = Box::new(0);
    let result = dispatch(&param);
    eprintln!("dispatch returned: {result:?}");
    eprintln!("param is now pointing to: {param:p}")
}
