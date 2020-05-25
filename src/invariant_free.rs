/// A type does not have any internal invariants that must be held in order for the
/// type to be used safely _and_ correctly.
///
/// Such data is often known as "plain ol' data" (a.k.a. POD) types.
///
/// # Examples
///
/// For example, `u8`s are `InvariantFree` since they are valid no matter what value they contain.
/// Conversely, `NonZeroU8` is not `InvariantFree` since it requires that it cannot be made zero.
///
/// It is important to note that this trait assumes that types keep no invariants even if violating
/// those invariants is memory safe. For example, the following type is _not_ `InvariantFree`:
///
/// ```rust
/// // A `u8` that is never `5`
/// struct NotFive(u8);
/// impl NotFive {
///     fn inc(&mut self) {
///         if self.0 == 4 {
///             self.0 = 6;
///         } else {
///             self.0 = self.0.wrapping_add(1);
///         }
///     }
/// }
/// ```
pub unsafe trait InvariantFree {}

macro_rules! invariant_free_impl {
    ($($type:ty),*) => {
        $(unsafe impl InvariantFree for $type {})*
    };
}

invariant_free_impl!(
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

unsafe impl<T> InvariantFree for *const T {}
unsafe impl<T> InvariantFree for *mut T {}
unsafe impl<T> InvariantFree for Option<core::ptr::NonNull<T>> {}

unsafe impl<T: InvariantFree> InvariantFree for [T; 0] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 1] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 2] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 3] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 4] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 5] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 6] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 7] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 8] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 9] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 10] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 11] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 12] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 13] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 14] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 15] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 16] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 17] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 18] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 19] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 20] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 21] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 22] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 23] {}
unsafe impl<T: InvariantFree> InvariantFree for [T; 24] {}
