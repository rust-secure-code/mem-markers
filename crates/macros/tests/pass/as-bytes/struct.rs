use mem_markers::{AsBytes, FixedLayout, InvariantFree, NoUninit};

#[derive(AsBytes, FixedLayout, InvariantFree, NoUninit)]
#[repr(C)]
struct Foo {
    a: u8,
    b: u8,
}

fn main() {
    fn ensure<T: AsBytes>() {}
    ensure::<Foo>();
}
