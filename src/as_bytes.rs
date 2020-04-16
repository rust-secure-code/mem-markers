/// A type which can be legally converted to raw bytes.
///
/// It is necessary for this type have a fixed layout and not
/// have any uninitialized bytes hence it requires those two traits
pub unsafe trait AsBytes: crate::NoUninit + crate::FixedLayout {}

#[doc(hidden)]
pub fn ensure<T: AsBytes>() {}
