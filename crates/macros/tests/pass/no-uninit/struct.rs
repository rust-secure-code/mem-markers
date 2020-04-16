use macros::NoUninit;

#[derive(NoUninit)]
#[repr(C)]
struct Foo {
    a: u8,
}

fn main() {
    mem_markers::no_uninit::ensure::<Foo>();
}
