fn main() {
    let _: &mut (dyn ?Sized + !Send) = &mut {};
}
