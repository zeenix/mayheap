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

// Re-exports for the macros.
#[doc(hidden)]
pub mod reexports {
    #[cfg(feature = "alloc")]
    pub extern crate alloc;
    pub use paste;
}

pub mod vec;
pub use vec::Vec;

pub mod string;
pub use string::String;

mod error;
pub use error::{Error, Result};

#[cfg(any(
    all(feature = "portable-atomic", feature = "heapless"),
    feature = "alloc"
))]
pub mod boxed;

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    #[test]
    fn serde() {
        let s = crate::String::<16>::try_from("hello").unwrap();
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, r#""hello""#);
        let s2: crate::String<16> = serde_json::from_str(&json).unwrap();
        assert_eq!(s, s2);

        let v = crate::Vec::<_, 10>::from_slice(&[1, 2, 3, 4, 5]).unwrap();
        let json = serde_json::to_string(&v).unwrap();
        assert_eq!(json, r#"[1,2,3,4,5]"#);
        let v2: crate::Vec<u8, 5> = serde_json::from_str(&json).unwrap();
        assert_eq!(v, v2);

        // Doesn't fit so should fail with `heapless` but not with `alloc`.
        let res = serde_json::from_str::<crate::Vec<u8, 1>>(&json);
        #[cfg(not(feature = "alloc"))]
        res.unwrap_err();
        #[cfg(feature = "alloc")]
        res.unwrap();
    }
}
