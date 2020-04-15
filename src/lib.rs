/// A marker trait representing types that have fixed layouts in memory
///
/// Types are `FixedLayout` if they have a well-defined layout in memory.
/// This is usually reserved for primitive types like numbers and bool
/// as well as complex type that are not `#[repr(rust)]` (the default)
/// representation for Rust types
pub unsafe trait FixedLayout {}
pub fn ensure_fixed_layout<T: FixedLayout>() {}

macro_rules! fixed_layout_impl {
    ($($type:ty),*) => {
        $(unsafe impl FixedLayout for $type {})*
    };
}
fixed_layout_impl!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool);
