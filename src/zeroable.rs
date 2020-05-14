/// A type which can be legally instantiate from zeroed out memory
///
/// It is not necessary for this type have a fixed layout nor
/// be byte complete just that its layout is guranteed to be valid
/// if all zeros.
pub unsafe trait Zeroable {}

macro_rules! from_bytes_impl {
    ($($type:ty),*) => {
        $(unsafe impl Zeroable for $type {})*
    };
}

from_bytes_impl!(
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
    isize,
    usize,
    Option<core::num::NonZeroI8>,
    Option<core::num::NonZeroU8>,
    Option<core::num::NonZeroI16>,
    Option<core::num::NonZeroU16>,
    Option<core::num::NonZeroI32>,
    Option<core::num::NonZeroU32>,
    Option<core::num::NonZeroI64>,
    Option<core::num::NonZeroU64>,
    Option<core::num::NonZeroI128>,
    Option<core::num::NonZeroU128>
);

unsafe impl<T> Zeroable for Option<core::ptr::NonNull<T>> {}
unsafe impl<T: Zeroable> Zeroable for [T; 0] {}
unsafe impl<T: Zeroable> Zeroable for [T; 1] {}
unsafe impl<T: Zeroable> Zeroable for [T; 2] {}
unsafe impl<T: Zeroable> Zeroable for [T; 3] {}
unsafe impl<T: Zeroable> Zeroable for [T; 4] {}
unsafe impl<T: Zeroable> Zeroable for [T; 5] {}
unsafe impl<T: Zeroable> Zeroable for [T; 6] {}
unsafe impl<T: Zeroable> Zeroable for [T; 7] {}
unsafe impl<T: Zeroable> Zeroable for [T; 8] {}
unsafe impl<T: Zeroable> Zeroable for [T; 9] {}
unsafe impl<T: Zeroable> Zeroable for [T; 10] {}
unsafe impl<T: Zeroable> Zeroable for [T; 11] {}
unsafe impl<T: Zeroable> Zeroable for [T; 12] {}
unsafe impl<T: Zeroable> Zeroable for [T; 13] {}
unsafe impl<T: Zeroable> Zeroable for [T; 14] {}
unsafe impl<T: Zeroable> Zeroable for [T; 15] {}
unsafe impl<T: Zeroable> Zeroable for [T; 16] {}
unsafe impl<T: Zeroable> Zeroable for [T; 17] {}
unsafe impl<T: Zeroable> Zeroable for [T; 18] {}
unsafe impl<T: Zeroable> Zeroable for [T; 19] {}
unsafe impl<T: Zeroable> Zeroable for [T; 20] {}
unsafe impl<T: Zeroable> Zeroable for [T; 21] {}
unsafe impl<T: Zeroable> Zeroable for [T; 22] {}
unsafe impl<T: Zeroable> Zeroable for [T; 23] {}
unsafe impl<T: Zeroable> Zeroable for [T; 24] {}
