/// A type which has no uninitialized bytes
///
/// A type usually has uninitialized bytes in two ways:
/// * Explicitly uninitialized bytes: usually done through [`MaybeUninit`]
/// * Padding
///
/// As such, types implementing `NoUninit` must have none of the above.
///
/// [`MaybeUninit`](https://doc.rust-lang.org/stable/core/mem/union.MaybeUninit.html)
pub unsafe trait NoUninit {}

macro_rules! no_uninit_impl {
    ($($type:ty),*) => {
        $(unsafe impl NoUninit for $type {})*
    };
}

no_uninit_impl!(
    (),
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

// Note: all types guarantee their size is a multiple of their alignment,
// so a slice [T; N] can never contain padding that the type T doesn't itself contain.
unsafe impl<T: NoUninit> NoUninit for [T; 0] {}
unsafe impl<T: NoUninit> NoUninit for [T; 1] {}
unsafe impl<T: NoUninit> NoUninit for [T; 2] {}
unsafe impl<T: NoUninit> NoUninit for [T; 3] {}
unsafe impl<T: NoUninit> NoUninit for [T; 4] {}
unsafe impl<T: NoUninit> NoUninit for [T; 5] {}
unsafe impl<T: NoUninit> NoUninit for [T; 6] {}
unsafe impl<T: NoUninit> NoUninit for [T; 7] {}
unsafe impl<T: NoUninit> NoUninit for [T; 8] {}
unsafe impl<T: NoUninit> NoUninit for [T; 9] {}
unsafe impl<T: NoUninit> NoUninit for [T; 10] {}
unsafe impl<T: NoUninit> NoUninit for [T; 11] {}
unsafe impl<T: NoUninit> NoUninit for [T; 12] {}
unsafe impl<T: NoUninit> NoUninit for [T; 13] {}
unsafe impl<T: NoUninit> NoUninit for [T; 14] {}
unsafe impl<T: NoUninit> NoUninit for [T; 15] {}
unsafe impl<T: NoUninit> NoUninit for [T; 16] {}
unsafe impl<T: NoUninit> NoUninit for [T; 17] {}
unsafe impl<T: NoUninit> NoUninit for [T; 18] {}
unsafe impl<T: NoUninit> NoUninit for [T; 19] {}
unsafe impl<T: NoUninit> NoUninit for [T; 20] {}
unsafe impl<T: NoUninit> NoUninit for [T; 21] {}
unsafe impl<T: NoUninit> NoUninit for [T; 22] {}
unsafe impl<T: NoUninit> NoUninit for [T; 23] {}
unsafe impl<T: NoUninit> NoUninit for [T; 24] {}
