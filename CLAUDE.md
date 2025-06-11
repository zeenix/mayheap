# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Commands

Build and test:
```bash
cargo build
cargo test --features serde
cargo test --no-default-features --features heapless,serde
```

Linting and formatting:
```bash
cargo +nightly fmt
cargo clippy
cargo doc --all-features
```

## Architecture

This crate provides abstractions over `alloc` and `heapless` collections,
allowing code to work with both heap-allocated and stack-allocated data
structures through a unified API.

### Key Design Patterns

- **Feature-based compilation**: Either `alloc` (default) or `heapless` feature
  must be enabled, but not both simultaneously
- **Capacity parameterization**: All types use const generic `N` parameter for
  capacity control - initial capacity for `alloc`, maximum capacity for
  `heapless`
- **Fallible operations**: Methods return `Result<T, Error>` to handle capacity
  limits in `heapless` mode, though they're infallible with `alloc`
- **Backend abstraction**: Internal `Inner<T, N>` type aliases provide
  conditional compilation between `alloc::vec::Vec<T>` and `heapless::Vec<T, N>`

### Core Modules

- `vec.rs`: Vec implementation with unified API over both backends
- `string.rs`: String implementation with unified API over both backends  
- `error.rs`: Error types (`BufferOverflow`, `Utf8Error`) and Result alias
- `lib.rs`: Feature gates, re-exports, and serde integration tests

The crate uses `#![no_std]` and requires explicit `extern crate alloc` when
alloc feature is enabled.
