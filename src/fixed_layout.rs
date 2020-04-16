/// A marker trait representing types that have fixed layouts in memory
///
/// Types are `FixedLayout` if they have a well-defined layout in memory.
/// This is usually reserved for primitive types like numbers and bool
/// as well as complex type that are not `#[repr(rust)]` (the default)
/// representation for Rust types
pub unsafe trait FixedLayout {}

#[doc(hidden)]
pub fn ensure<T: FixedLayout>() {}

macro_rules! fixed_layout_impl {
    ($($type:ty),*) => {
        $(unsafe impl FixedLayout for $type {})*
    };
}

fixed_layout_impl!(
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    usize,
    isize,
    bool,
    char,
    core::num::NonZeroI8,
    core::num::NonZeroU8,
    core::num::NonZeroI16,
    core::num::NonZeroU16,
    core::num::NonZeroI32,
    core::num::NonZeroU32,
    core::num::NonZeroI64,
    core::num::NonZeroU64,
    core::num::NonZeroI128,
    core::num::NonZeroU128,
    Option<core::num::NonZeroI8>,
    Option<core::num::NonZeroU8>,
    Option<core::num::NonZeroI16>,
    Option<core::num::NonZeroU16>,
    Option<core::num::NonZeroI32>,
    Option<core::num::NonZeroU32>,
    Option<core::num::NonZeroI64>,
    Option<core::num::NonZeroU64>,
    Option<core::num::NonZeroI128>,
    Option<core::num::NonZeroU128>,
    f32,
    f64
);

unsafe impl<T> FixedLayout for *const T {}
unsafe impl<T> FixedLayout for *mut T {}
unsafe impl<T> FixedLayout for &T {}
unsafe impl<T> FixedLayout for &mut T {}
unsafe impl<T> FixedLayout for core::ptr::NonNull<T> {}
unsafe impl<T> FixedLayout for Option<core::ptr::NonNull<T>> {}

unsafe impl<T: FixedLayout> FixedLayout for [T; 0] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 1] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 2] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 3] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 4] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 5] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 6] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 7] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 8] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 9] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 10] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 11] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 12] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 13] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 14] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 15] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 16] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 17] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 18] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 19] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 20] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 21] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 22] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 23] {}
unsafe impl<T: FixedLayout> FixedLayout for [T; 24] {}
