use macros::FixedLayout;

#[derive(FixedLayout)]
#[repr(C)]
struct Foo<'a> {
    a: &'a Other,
}
struct Other {
    b: u8,
}

fn main() {}
