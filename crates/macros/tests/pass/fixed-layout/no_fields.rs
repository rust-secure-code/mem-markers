use macros::FixedLayout;

#[derive(FixedLayout)]
struct Foo {}

fn main() {
    mem_markers::fixed_layout::ensure::<Foo>();
}
