//! Marker traits for establishing certain invariants of how a type is represented in memory

#![deny(missing_docs)]

#[doc(hidden)]
pub mod as_bytes;
#[doc(hidden)]
pub mod byte_complete;
#[doc(hidden)]
pub mod fixed_layout;
#[doc(hidden)]
pub mod from_bytes;
#[doc(hidden)]
pub mod no_uninit;

#[doc(inline)]
pub use as_bytes::AsBytes;
#[doc(inline)]
pub use byte_complete::ByteComplete;
#[doc(inline)]
pub use fixed_layout::FixedLayout;
#[doc(inline)]
pub use from_bytes::FromBytes;
#[doc(inline)]
pub use no_uninit::NoUninit;

pub use mem_markers_macros::*;