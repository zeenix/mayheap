#![no_std]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs
)]
#![warn(unreachable_pub)]
#![doc = include_str!("../README.md")]

#[cfg(all(not(feature = "alloc"), not(feature = "heapless")))]
compile_error!("Either the `alloc` or `heapless` feature must be enabled");

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod vec;
#[cfg(feature = "alloc")]
pub use vec::Vec;

/// `Vec` is just a type alias with `heapless` feature.
#[cfg(not(feature = "alloc"))]
pub type Vec<T, const N: usize> = heapless::Vec<T, N>;
