/// A type which can be legally converted to raw bytes.
///
/// It is necessary for this type have a fixed layout and not
/// have any uninitialized bytes hence it requires those two traits
pub unsafe trait AsBytes: crate::NoUninit + crate::FixedLayout {}

macro_rules! as_bytes_impl {
    ($($type:ty),*) => {
        $(unsafe impl AsBytes for $type {})*
    };
}

as_bytes_impl!(
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

unsafe impl<T> AsBytes for *const T {}
unsafe impl<T> AsBytes for *mut T {}
unsafe impl<T> AsBytes for &T {}
unsafe impl<T> AsBytes for &mut T {}
unsafe impl<T> AsBytes for core::ptr::NonNull<T> {}
unsafe impl<T> AsBytes for Option<core::ptr::NonNull<T>> {}

unsafe impl<T: AsBytes> AsBytes for [T; 0] {}
unsafe impl<T: AsBytes> AsBytes for [T; 1] {}
unsafe impl<T: AsBytes> AsBytes for [T; 2] {}
unsafe impl<T: AsBytes> AsBytes for [T; 3] {}
unsafe impl<T: AsBytes> AsBytes for [T; 4] {}
unsafe impl<T: AsBytes> AsBytes for [T; 5] {}
unsafe impl<T: AsBytes> AsBytes for [T; 6] {}
unsafe impl<T: AsBytes> AsBytes for [T; 7] {}
unsafe impl<T: AsBytes> AsBytes for [T; 8] {}
unsafe impl<T: AsBytes> AsBytes for [T; 9] {}
unsafe impl<T: AsBytes> AsBytes for [T; 10] {}
unsafe impl<T: AsBytes> AsBytes for [T; 11] {}
unsafe impl<T: AsBytes> AsBytes for [T; 12] {}
unsafe impl<T: AsBytes> AsBytes for [T; 13] {}
unsafe impl<T: AsBytes> AsBytes for [T; 14] {}
unsafe impl<T: AsBytes> AsBytes for [T; 15] {}
unsafe impl<T: AsBytes> AsBytes for [T; 16] {}
unsafe impl<T: AsBytes> AsBytes for [T; 17] {}
unsafe impl<T: AsBytes> AsBytes for [T; 18] {}
unsafe impl<T: AsBytes> AsBytes for [T; 19] {}
unsafe impl<T: AsBytes> AsBytes for [T; 20] {}
unsafe impl<T: AsBytes> AsBytes for [T; 21] {}
unsafe impl<T: AsBytes> AsBytes for [T; 22] {}
unsafe impl<T: AsBytes> AsBytes for [T; 23] {}
unsafe impl<T: AsBytes> AsBytes for [T; 24] {}
