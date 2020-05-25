//! Marker traits for establishing certain invariants of how a type is represented in memory

#![no_std]
#![deny(missing_docs)]

mod as_bytes;
mod byte_complete;
mod fixed_layout;
mod from_bytes;
mod invariant_free;
mod no_uninit;
mod zeroable;

#[doc(inline)]
pub use as_bytes::AsBytes;
#[doc(inline)]
pub use byte_complete::ByteComplete;
#[doc(inline)]
pub use fixed_layout::FixedLayout;
#[doc(inline)]
pub use from_bytes::FromBytes;
#[doc(inline)]
pub use invariant_free::InvariantFree;
#[doc(inline)]
pub use no_uninit::NoUninit;
#[doc(inline)]
pub use zeroable::Zeroable;

pub use mem_markers_macros::*;
