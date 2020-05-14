use mem_markers::{AsBytes, FixedLayout, NoUninit};

#[derive(AsBytes, FixedLayout, NoUninit)]
#[repr(C)]
struct Foo {
    a: u8,
    b: u8,
}

fn main() {
    fn ensure<T: AsBytes>() {}
    ensure::<Foo>();
}
