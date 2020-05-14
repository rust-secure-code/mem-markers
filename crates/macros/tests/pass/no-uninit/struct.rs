use mem_markers::NoUninit;

#[derive(NoUninit)]
#[repr(C)]
struct Foo {
    a: u8,
}

fn main() {
    fn ensure<T: NoUninit>() {}
    ensure::<Foo>();
}
