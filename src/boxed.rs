//! Abstraction over `heapless::pool::boxed` and `alloc::boxed`.
//!
//! The API is modeled after `heapless::pool::boxed`. This module is only available when either:
//!
//! - `alloc` feature is enabled, or
//! - `heapless` and `portable-atomic` features are enabled.
//!
//! # Usage
//!
//! ```
//! use mayheap::{box_pool, boxed::{BoxPool, Box}};
//!
//! extern crate alloc;
//!
//! // Create a pool for u32 type with a capacity of 10.
//! box_pool!(MyBoxPool: u32, 2);
//! // Initialize the pool.
//! MyBoxPool.init();
//!
//! // Allocate a new boxed value from the pool.
//! let mut boxed = MyBoxPool.alloc(42).unwrap();
//! assert_eq!(*boxed, 42);
//!
//! // Calling `init` again is a no-op.
//! MyBoxPool.init();
//! assert_eq!(*boxed, 42);
//!
//! // Let's mutate the boxed value.
//! *boxed = 100;
//! assert_eq!(*boxed, 100);
//!
//! // Let's allocate more.
//! let _boxed = MyBoxPool.alloc(43).unwrap();
//!
//! #[cfg(feature = "alloc")]
//! {
//!     // This will work fine since capacity (which is 2 here) is irrelevant when using alloc.
//!     let boxed = MyBoxPool.alloc(44).unwrap();
//!     assert_eq!(*boxed, 44);
//! }
//! #[cfg(feature = "heapless")]
//! {
//!     // This will not.
//!     let boxed = MyBoxPool.alloc(45);
//!     assert!(boxed.is_none());
//! }
//! ```

use core::ops::{Deref, DerefMut};

/// A singleton that manages pool::boxed::Box-es.
///
/// Don't implement this trait directly. Use [`box_pool!`] to create an implementation.
pub trait BoxPool {
    /// The data type managed by the memory pool.
    type Data;
    /// The implementation-specific type of the boxed value.
    type BoxedValue: DerefMut<Target = Self::Data>;

    /// Allocates a new boxed value from the pool.
    fn alloc(&self, value: Self::Data) -> Option<Box<Self>>
    where
        Self: Sized;
}

/// A boxed value managed by a [`BoxPool`].
#[derive(Debug, PartialEq, Clone)]
pub struct Box<P: BoxPool>(P::BoxedValue);

impl<P: BoxPool> Box<P> {
    /// Allocates a new boxed value from the pool.
    pub fn new(value: P::BoxedValue) -> Self {
        Self(value)
    }
}

impl<P: BoxPool> Deref for Box<P> {
    type Target = P::Data;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<P: BoxPool> DerefMut for Box<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

/// Creates a new BoxPool singleton with the given $name that manages the specified $data_type
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! box_pool {
    ($name:ident: $ty:ty, $capacity:expr) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name;

        impl $crate::boxed::BoxPool for $name {
            type Data = $ty;
            type BoxedValue = alloc::boxed::Box<$ty>;

            fn alloc(&self, value: Self::Data) -> Option<$crate::boxed::Box<Self>> {
                Some($crate::boxed::Box::new(alloc::boxed::Box::new(value)))
            }
        }

        impl $name {
            pub fn init(&self) {
                // No initialization needed for `alloc` allocator.
            }
        }

        $crate::paste::paste! {
            // Let's use the $capacity variable so callers don't get "unused const" warnings.
            #[allow(non_upper_case_globals, dead_code)]
            const [<__dummy__ $name>]: () = {
                let _ = $capacity;
            };
        }
    };
}

/// Creates a new BoxPool singleton with the given $name that manages the specified $data_type
#[cfg(not(feature = "alloc"))]
#[macro_export]
macro_rules! box_pool {
    ($name:ident: $ty:ty, $capacity:expr) => {
        $crate::paste::paste! {
            heapless::box_pool!([<$name Pool>]: $ty);

            #[derive(Debug, Clone, PartialEq)]
            pub struct $name;

            impl $crate::boxed::BoxPool for $name {
                type Data = $ty;
                type BoxedValue = heapless::pool::boxed::Box<[<$name Pool>]>;

                fn alloc(&self, value: Self::Data) -> Option<$crate::boxed::Box<Self>> {
                    [<$name Pool>].alloc(value).ok().map($crate::boxed::Box::new)
                }
            }

            impl $name {
                pub fn init(&self) {
                    use core::sync::atomic::{AtomicBool, Ordering};
                    use heapless::pool::boxed::BoxBlock;

                    static initialized: AtomicBool = AtomicBool::new(false);

                    if !initialized.load(Ordering::Acquire) {
                        let blocks: &'static mut [BoxBlock<$ty>] = {
                            #[allow(clippy::declare_interior_mutable_const)]
                            const BLOCK: BoxBlock<$ty> = BoxBlock::new();
                            static mut BLOCKS: [BoxBlock<$ty>; $capacity] = [BLOCK; $capacity];
                            unsafe { core::ptr::addr_of_mut!(BLOCKS).as_mut().unwrap() }
                        };
                        for block in blocks {
                           [<$name Pool>].manage(block);
                        }

                        initialized.store(true, Ordering::Release);
                    }
                }
            }
        }
    };
}
