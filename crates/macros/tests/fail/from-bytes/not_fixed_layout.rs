use mem_markers::*;

#[derive(FromBytes, ByteComplete)]
#[repr(C)]
struct Foo {
    a: u8,
}

fn main() {}
