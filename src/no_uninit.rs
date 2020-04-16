/// A type which has no uninitialized bytes
pub unsafe trait NoUninit {}

#[doc(hidden)]
pub fn ensure<T: NoUninit>() {}

macro_rules! no_uninit_impl {
    ($($type:ty),*) => {
        $(unsafe impl NoUninit for $type {})*
    };
}

no_uninit_impl!(
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

unsafe impl<T> NoUninit for *const T {}
unsafe impl<T> NoUninit for *mut T {}
unsafe impl<T> NoUninit for &T {}
unsafe impl<T> NoUninit for &mut T {}
unsafe impl<T> NoUninit for core::ptr::NonNull<T> {}
unsafe impl<T> NoUninit for Option<core::ptr::NonNull<T>> {}
