use macros::*;

#[derive(FromBytes, ByteComplete)]
#[repr(C)]
struct Foo {
    a: u8,
}

fn main() {}
