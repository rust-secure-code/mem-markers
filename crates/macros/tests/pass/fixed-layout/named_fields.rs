use mem_markers::FixedLayout;

#[derive(FixedLayout)]
#[repr(C)]
struct Foo {
    a: u8,
}

fn main() {
    fn ensure<T: FixedLayout>() {}
    ensure::<Foo>();
}
