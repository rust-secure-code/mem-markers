/// A marker trait representing types are valid no matter what byte values represent them in memory
///
/// Types like `u8` are `ByteComplete` because no matter what value the byte that represents the `u8`
/// is in memory, it's guranteed to be a valid `u8`. `bool` is not `ByteComplete` because only the bytes
/// `0b1` and `0b0` are valid `bool`s.
pub unsafe trait ByteComplete {}

#[doc(hidden)]
pub fn ensure<T: ByteComplete>() {}

macro_rules! byte_complete_impl {
    ($($type:ty),*) => {
        $(unsafe impl ByteComplete for $type {})*
    };
}

byte_complete_impl!(
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

unsafe impl<T> ByteComplete for Option<core::ptr::NonNull<T>> {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 0] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 1] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 2] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 3] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 4] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 5] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 6] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 7] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 8] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 9] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 10] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 11] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 12] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 13] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 14] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 15] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 16] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 17] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 18] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 19] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 20] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 21] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 22] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 23] {}
unsafe impl<T: ByteComplete> ByteComplete for [T; 24] {}
