<p align="center">
  <a href="https://github.com/zeenix/mayheap/actions/workflows/rust.yml">
    <img alt="Build Status" src="https://github.com/zeenix/mayheap/actions/workflows/rust.yml/badge.svg">
  </a>
  <a href="https://docs.rs/mayheap/">
    <img alt="API Documentation" src="https://docs.rs/mayheap/badge.svg">
  </a>
  <a href="https://crates.io/crates/mayheap">
    <img alt="crates.io" src="https://img.shields.io/crates/v/mayheap">
  </a>
</p>

<h1 align="center">mayheap</h1>

This crate provides an abstraction over `alloc` and `heapless` crates. You'd use this crate where
you'd normally use `alloc` but you want to also support `heapless` as an alternative for baremetal
low-end targets (think microcontrollers).

All types have a generic const parameter that controls how much space to allocate for it. In case of
`heapless` this is the maximum capacity of the type. In case of `alloc` this is the initial capacity
of the type. Hence, all fallible operations are in reality infallible and all unsafe methods are
safe in the latter case. Note however that this only includes failures and unsafety due to buffer
overflows.

## Usage

The usage is very similar to the `heapless` types:

```rust
use mayheap::{Vec, String};

// Vec

let mut vec = Vec::<_, 4>::new();
vec.push(1).unwrap();
vec.push(2).unwrap();

assert_eq!(vec.len(), 2);
assert_eq!(vec[0], 1);

assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.len(), 1);

vec[0] = 7;
assert_eq!(vec[0], 7);

vec.extend([1, 2, 3]);

for x in &vec {
    println!("{x}");
}
assert_eq!(vec, [7, 1, 2, 3]);

for x in vec.into_iter() {
    println!("{x}");
}

// String

let mut s = String::<16>::try_from("hello").unwrap();
s.push_str(" world").unwrap();
assert_eq!(s, "hello world");

// Going beyond the capacity will succeed in case of `alloc`..
#[cfg(feature = "alloc")]
s.push_str("mooooooooooooooooooooooooore").unwrap();

// ..but result in an error in case of `heapless`.
#[cfg(feature = "heapless")]
s.push_str("mooooooooooooooooooooooooore").unwrap_err();
```

## Features

* `alloc` (default): Enables `alloc` backend.
* `heapless`: Enables `heapless` backend.
* `serde`: Implement `serde::{Serialize, Deserialize}` for all types.

Either `alloc` or `heapless` feature must be enabled. If both are enabled, `alloc` will be used and
`heapless` dependency gets pulled in unnecessarily. So don't do that! 😄

## License

MIT

## TODO

* Update to heapless' next release when it's out and remove `vec::IntoIter` manual implementation
* `Box<T>`
* `Arc<T>`
* `Deque<T>`
* `Map<K, V>`
