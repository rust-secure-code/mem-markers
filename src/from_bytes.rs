/// A type which can be safely _and_ correctly instantiated from raw
/// bytes _and_ then safely _and_ correctly used.
///
/// Types that implement `FromBytes` require the following traits to also
/// be implemented:
/// * [`FixedLayout`]: in order to get reliable results, the type
/// must have a well known layout.
/// * [`ByteComplete`]: in order to be able to turn _any_ properly sized
/// aligned byte array into the type, the type must be byte complete.
/// byte complete hence it requires the type implements
/// * [`InvariantFree`]: The type must not rely on any invariants to be confirmed
/// before the type can be used safely _and_ correctly.
///
/// See those types documentation for information on the invariants those
/// traits represent.
///
/// [`FromBytes`] is the logical opposite as [`AsBytes`].
///
/// [`FromBytes`]: trait.FromBytes.html
/// [`AsBytes`]: trait.AsBytes.html
/// [`ByteComplete`]: trait.ByteComplete.html
/// [`FixedLayout`]: trait.FixedLayout.html
/// [`InvariantFree`]: trait.InvariantFree.html
pub unsafe trait FromBytes:
    crate::InvariantFree + crate::ByteComplete + crate::FixedLayout
{
}

macro_rules! from_bytes_impl {
    ($($type:ty),*) => {
        $(unsafe impl FromBytes for $type {})*
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

unsafe impl<T> FromBytes for Option<core::ptr::NonNull<T>> {}
unsafe impl<T: FromBytes> FromBytes for [T; 0] {}
unsafe impl<T: FromBytes> FromBytes for [T; 1] {}
unsafe impl<T: FromBytes> FromBytes for [T; 2] {}
unsafe impl<T: FromBytes> FromBytes for [T; 3] {}
unsafe impl<T: FromBytes> FromBytes for [T; 4] {}
unsafe impl<T: FromBytes> FromBytes for [T; 5] {}
unsafe impl<T: FromBytes> FromBytes for [T; 6] {}
unsafe impl<T: FromBytes> FromBytes for [T; 7] {}
unsafe impl<T: FromBytes> FromBytes for [T; 8] {}
unsafe impl<T: FromBytes> FromBytes for [T; 9] {}
unsafe impl<T: FromBytes> FromBytes for [T; 10] {}
unsafe impl<T: FromBytes> FromBytes for [T; 11] {}
unsafe impl<T: FromBytes> FromBytes for [T; 12] {}
unsafe impl<T: FromBytes> FromBytes for [T; 13] {}
unsafe impl<T: FromBytes> FromBytes for [T; 14] {}
unsafe impl<T: FromBytes> FromBytes for [T; 15] {}
unsafe impl<T: FromBytes> FromBytes for [T; 16] {}
unsafe impl<T: FromBytes> FromBytes for [T; 17] {}
unsafe impl<T: FromBytes> FromBytes for [T; 18] {}
unsafe impl<T: FromBytes> FromBytes for [T; 19] {}
unsafe impl<T: FromBytes> FromBytes for [T; 20] {}
unsafe impl<T: FromBytes> FromBytes for [T; 21] {}
unsafe impl<T: FromBytes> FromBytes for [T; 22] {}
unsafe impl<T: FromBytes> FromBytes for [T; 23] {}
unsafe impl<T: FromBytes> FromBytes for [T; 24] {}
