//! Memory marking traits

#![deny(missing_docs)]

#[doc(hidden)]
pub mod byte_complete;
#[doc(hidden)]
pub mod fixed_layout;
#[doc(hidden)]
pub mod from_bytes;

#[doc(inline)]
pub use byte_complete::ByteComplete;
#[doc(inline)]
pub use fixed_layout::FixedLayout;
#[doc(inline)]
pub use from_bytes::FromBytes;
