#[derive(Debug)]
#[repr(packed)]
enum COption<T> {
    None,
    Some(T),
}

fn main() {
}