use mem_markers::ByteComplete;

#[derive(ByteComplete)]
struct Foo {
    a: Other,
}

struct Other {}
fn main() {}
