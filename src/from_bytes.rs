/// A type which can be legally converted from raw bytes.
///
/// It is necessary for this type have a fixed layout and be
/// byte complete hence it requires those two traits are implemented
pub unsafe trait FromBytes: crate::ByteComplete + crate::FixedLayout {}

#[doc(hidden)]
pub fn ensure<T: FromBytes>() {}
