type Fun = unsafe extern "C" fn();

struct Foo(Fun);

static FOO: &Foo = &Foo(BAR);

unsafe extern "C" {
    static BAR: Fun;
}