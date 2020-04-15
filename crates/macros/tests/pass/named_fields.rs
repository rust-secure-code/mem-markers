use macros::FixedLayout;

#[derive(FixedLayout)]
#[repr(C)]
struct Foo {
    a: u8,
}

fn main() {
    mem_markers::fixed_layout::ensure::<Foo>();
}
