use macros::*;

#[derive(AsBytes, FixedLayout, NoUninit)]
#[repr(C)]
struct Foo {
    a: u8,
    b: u8,
}

fn main() {
    mem_markers::as_bytes::ensure::<Foo>();
}
