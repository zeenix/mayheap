# heapfull

This crate provides an abstraction over `alloc` and `heapless` crates. You'd use this crate where
you'd normally use `alloc` but you want to also support `heapless` as an alternative for baremetal
low-end targets (think microcontrollers).

All types have a generic const parameter that controls how much space to allocate for it. In case of
`heapless` this is the maximum capacity of the type. In case of `alloc` this is the initial capacity
of the type.
