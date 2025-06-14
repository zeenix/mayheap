//! Abstraction over `heapless::pool::boxed` and `alloc::boxed`.
//!
//! The API is modeled after `heapless::pool::boxed` but simpler. This module is only available
//! when either:
//!
//! - `alloc` feature is enabled, or
//! - `heapless` and `portable-atomic` features are enabled.
//!
//! # Usage
//!
//! ```
//! use mayheap::{box_pool, boxed::{BoxPool, Box}};
//!
//! // Create a pool for u32 type with a capacity of 2.
//! box_pool!(MyBoxPool: u32, 2);
//!
//! // Allocate a new boxed value from the pool.
//! let mut boxed = MyBoxPool.alloc(42).unwrap();
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
//!     // This will work fine since capacity (which is 2 here) is irrelevant with `alloc` feature.
//!     let boxed = MyBoxPool.alloc(44).unwrap();
//!     assert_eq!(*boxed, 44);
//! }
//! #[cfg(feature = "heapless")]
//! {
//!     // This will not.
//!     let res = MyBoxPool.alloc(45);
//!     assert_eq!(res, Err(45));
//! }
//! ```

use core::ops::{Deref, DerefMut};

/// A singleton that manages `pool::boxed::Box`-es.
///
/// Don't implement this trait directly. Use [`crate::box_pool`] to create an implementation.
pub trait BoxPool {
    /// The data type managed by the memory pool.
    type Data;
    /// The implementation-specific type of the boxed value.
    type BoxedValue: DerefMut<Target = Self::Data>;

    /// Allocates a new boxed value from the pool.
    fn alloc(&self, value: Self::Data) -> Result<Box<Self>, Self::Data>
    where
        Self: Sized;
}

/// A boxed value managed by a [`BoxPool`].
#[derive(Debug, PartialEq, Eq, Clone)]
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
    ($visibility:vis $name:ident: $ty:ty, $capacity:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        $visibility struct $name;

        impl $crate::boxed::BoxPool for $name {
            type Data = $ty;
            type BoxedValue = $crate::reexports::alloc::boxed::Box<$ty>;

            fn alloc(&self, value: Self::Data) -> Result<$crate::boxed::Box<Self>, Self::Data> {
                Ok($crate::boxed::Box::new(
                    $crate::reexports::alloc::boxed::Box::new(value),
                ))
            }
        }

        $crate::reexports::paste::paste! {
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
    ($visibility:vis $name:ident: $ty:ty, $capacity:expr) => {
        $crate::reexports::paste::paste! {
            heapless::box_pool!([<$name Pool>]: $ty);

            #[derive(Debug, Clone, PartialEq, Eq)]
            $visibility struct $name;

            impl $crate::boxed::BoxPool for $name {
                type Data = $ty;
                type BoxedValue = heapless::pool::boxed::Box<[<$name Pool>]>;

                fn alloc(&self, value: Self::Data) -> Result<$crate::boxed::Box<Self>, $ty> {
                    $name.init();

                    [<$name Pool>].alloc(value).map($crate::boxed::Box::new)
                }
            }

            impl $name {
                fn init(&self) {
                    use portable_atomic::{AtomicU8, Ordering};
                    use heapless::pool::boxed::BoxBlock;

                    static STATE: AtomicU8 = AtomicU8::new(InitState::Uninitialized as u8);

                    match STATE
                        .compare_exchange(
                            InitState::Uninitialized as u8,
                            InitState::Initializing as u8,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                        .map(|state| state.into())
                        .map_err(|state| state.into())
                    {
                        Ok(InitState::Uninitialized) => {
                            // We won the race, initialize.
                            let blocks: &'static mut [BoxBlock<$ty>] = {
                                static mut BLOCKS: [BoxBlock<$ty>; $capacity] = [const { BoxBlock::new() }; $capacity];
                                unsafe { &mut BLOCKS }
                            };
                            for block in blocks {
                               [<$name Pool>].manage(block);
                            }
                            STATE.store(InitState::Initialized as u8, Ordering::Release);
                        }
                        Err(InitState::Initializing) => {
                            // Someone else is initializing, wait.
                            while STATE.load(Ordering::Acquire) == InitState::Initializing as u8 {
                                core::hint::spin_loop();
                            }
                        }
                        Err(InitState::Initialized) => {
                            // Already initialized.
                        }
                        // All other states should never happen.
                        _ => unreachable!(),
                    }

                    #[repr(u8)]
                    #[derive(PartialEq)]
                    enum InitState {
                        Uninitialized = 0,
                        Initializing = 1,
                        Initialized = 2,
                    }

                    impl From<u8> for InitState {
                        fn from(value: u8) -> Self {
                            match value {
                                0 => InitState::Uninitialized,
                                1 => InitState::Initializing,
                                2 => InitState::Initialized,
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
        }
    };
}
