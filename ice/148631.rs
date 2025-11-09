struct C;

struct S<T>(Option<T>);

trait Tr {
    type Out;
}

impl<T> Clone for S<C>
where
    S<T>: Tr<Out = T>,
{
    fn clone(&self) -> Self {
        *self
    }
}

fn main() {}