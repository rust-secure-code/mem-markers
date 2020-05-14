use mem_markers::*;

#[derive(FromBytes, ByteComplete, Zeroable)]
#[repr(C)]
struct Foo {
    a: u8,
}

fn main() {}
