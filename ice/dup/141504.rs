#![feature(closure_lifetime_binder)]
fn foo() {
    let a: for<'a> fn(&'a ()) -> &'a () = for<'a> |b: &'a ()| -> &'a () {
        const {
            let awd = ();
            let _: &'a () = &awd;
        };
        b
    };
}

fn main() {}