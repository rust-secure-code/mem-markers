use mem_markers::FixedLayout;

#[derive(FixedLayout)]
struct Foo {}

fn main() {
    fn ensure<T: FixedLayout>() {}
    ensure::<Foo>();
}
