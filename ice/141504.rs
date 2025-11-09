#![feature(closure_lifetime_binder)]

fn fails() {
    callback(for<'a> |y: &'a mut i32| -> () {
        let _: &'a mut i32 = y;
    });
}

fn callback(_: impl for<'a> FnOnce(&'a mut i32)) {}