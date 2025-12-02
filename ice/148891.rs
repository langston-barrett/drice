pub struct Char3(pub [i8; 3]);

macro_rules! values {
    ($($token:ident($value:literal) $(as $inner:ty)? => $attr:meta,)*) => {
        #[derive(Debug)]
        pub enum TokenKind {
            $(
                #[$attr]
                $token $(Char3([$inner, -3, $inner]))? = $value,
            )*
        }
    };
}

values!(STRING(1) as (String) => cfg(test),);

pub fn main() {}