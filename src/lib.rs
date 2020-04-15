pub unsafe trait FixedLayout {}

macro_rules! fixed_layout_impl {
    ($($type:ty),*) => {
        $(unsafe impl FixedLayout for $type {})*
    };
}
fixed_layout_impl!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
