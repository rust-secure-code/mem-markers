use mem_markers::FixedLayout;

#[derive(FixedLayout)]
#[repr(C)]
struct Foo {
    a: Other,
}

struct Other {}
fn main() {}
