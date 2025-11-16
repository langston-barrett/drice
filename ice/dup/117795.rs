const fn f() -> usize {}

fn main() {
    _ = [0; FnMut::call_mut(&mut f, ())];
}
