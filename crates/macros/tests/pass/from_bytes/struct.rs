use macros::*;

#[derive(FromBytes, FixedLayout, ByteComplete)]
#[repr(C)]
struct Foo {
    a: u8,
}

fn main() {
    mem_markers::from_bytes::ensure::<Foo>();
}
