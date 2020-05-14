use mem_markers::FixedLayout;

#[derive(FixedLayout)]
#[repr(C)]
struct Foo(u8);

fn main() {
    fn ensure<T: FixedLayout>() {}
    ensure::<Foo>();
}
