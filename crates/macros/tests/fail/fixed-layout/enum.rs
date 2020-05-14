use mem_markers::FixedLayout;

#[derive(FixedLayout)]
#[repr(C)]
enum Foo {
    A,
}

fn main() {}
