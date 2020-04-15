use macros::FixedLayout;

#[derive(FixedLayout)]
struct Foo {}

fn main() {
    mem_markers::ensure_fixed_layout::<Foo>();
}
