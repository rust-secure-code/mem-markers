use mem_markers::*;

#[derive(FromBytes, FixedLayout, ByteComplete, Zeroable)]
#[repr(C)]
struct Foo {
    a: u8,
}

fn main() {
    fn ensure<T: FromBytes>() {}
    ensure::<Foo>();
}
