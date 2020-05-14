use mem_markers::{ByteComplete, Zeroable};

#[derive(Zeroable, ByteComplete)]
struct Foo {
    a: Other,
}

struct Other {}
fn main() {}
