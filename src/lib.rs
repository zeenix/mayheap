#![no_std]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs
)]
#![warn(unreachable_pub)]
#![allow(clippy::result_unit_err, clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

#[cfg(all(not(feature = "alloc"), not(feature = "heapless")))]
compile_error!("Either the `alloc` or `heapless` feature must be enabled");

#[cfg(feature = "alloc")]
extern crate alloc;

mod vec;
#[cfg(feature = "alloc")]
pub use vec::Vec;
