use macros::FixedLayout;
// use mem_markers::FixedLayout;

#[derive(FixedLayout)]
struct Foo {
    f: u8,
}

#[derive(FixedLayout)]
#[repr(C)]
struct Bar {
    b: u8,
}

fn main() {}
