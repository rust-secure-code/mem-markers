pub unsafe trait FromBytes: crate::ByteComplete + crate::FixedLayout {}

#[doc(hidden)]
pub fn ensure<T: FromBytes>() {}
