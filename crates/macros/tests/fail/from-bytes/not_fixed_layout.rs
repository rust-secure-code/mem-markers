use mem_markers::*;

#[derive(FromBytes, ByteComplete, InvariantFree, Zeroable)]
#[repr(C)]
struct Foo {
    a: u8,
}

fn main() {}
