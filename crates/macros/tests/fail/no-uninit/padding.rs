use macros::NoUninit;

#[derive(NoUninit)]
struct Foo {
    a: u8,
    b: u16,
}

fn main() {}
