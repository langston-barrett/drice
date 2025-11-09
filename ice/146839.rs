#![feature(coroutines, stmt_expr_attributes)]

const _: for<'a> fn() -> i32 = #[coroutine] || -> i32 { yield 0; return 1; };

fn main(){}