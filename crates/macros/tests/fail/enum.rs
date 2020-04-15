use macros::FixedLayout;

#[derive(FixedLayout)]
#[repr(C)]
enum Foo {
    A,
}

fn main() {}
