use macros::ByteComplete;

#[derive(ByteComplete)]
struct Foo {
    a: Other,
}

struct Other {}
fn main() {}
