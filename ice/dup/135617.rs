trait Project {
    const ASSOC: usize;
}

fn foo()
where
    for<'a> (): Project,
{
    [(); <() as Project>::ASSOC];
}

pub fn main() {}
