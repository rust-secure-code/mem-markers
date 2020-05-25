# Mem Markers

A crate for traits that describe certain memory layout invariants of a given type along with derive macros for deriving these traits for custom types.

## Traits

This crate currently contains the following traits:
* `FixedLayout`: types that have a well-defined layout that can be relied on
* `NoUninit`: types that do not have any unintialized bytes 
* `ByteComplete`: types where any appropriately sized and aligned array of bytes is a valid representation of that type
* `Zeroable`: types where all zeros are a valid representation of the type in memory
* `FromBytes`: types where any appropriately sized and aligned array of bytes can be viewed as the type.
* `AsBytes`: The type can reliably be turned into a slice of bytes

## Examples

### Safe Transmute

```rust
fn safe_transmute<From: AsBytes, To: FromBytes>(from: From) -> To {
    let from = &std::mem::ManuallyDrop::new(from);
    assert!(std::mem::size_of::<From>() == std::mem::size_of::<To>(), "Cannot transmute to smaller type");
    assert!(std::mem::align_of::<From>() % std::mem::align_of::<To>() == 0, "Not aligned");
    
    unsafe { std::mem::transmute_copy(from) }
}
```

## Comparisons with Other Crates

There are several other crates that implement the same functionality as mem-markers even in part or in full. Here is a quick comparison. If you do not find your favorite crate here, please file an issue!

* [`typic`](https://github.com/jswrenn/typic) - `typic` aims to allow for "fearless transmute". `typic` is built on low-level primitives that can be used to build traits that represent the same invariants as `mem-markers` traits. Where the crate differs is that `typic` uses type-level programming for its implementation. This means that `typic` can generally express more subtle invariants than `mem-markers`, but its implementation is more complex. `typic` also does more than `mem-markers` by providing functionality around safe transmute. `mem-markers` is purposefully only marker traits and any actual implementation is left for other crates. 
* [`zerocopy`](https://docs.rs/zerocopy/0.3.0/zerocopy/) - `zerocopy` has similar levels of functionality to `typic` but takes an approach more akin to `mem-markers`. The main difference is that `mem-markers` aims at only exposing marker traits (at a more fine grained level than `zerocopy`). It would be possible to implement `zerocopy` in terms on `mem-markers`, exposing additional functionality on top of `mem-markers` marker traits.
* [bytemuck](https://crates.io/crates/bytemuck) - `bytemuck`, much like `zerocopy`, is a high level crate for exposing safe transmute functionality. Similarly, `bytemuck` could be implemented in terms of `mem-markers`.
* [`safe-transmute`](https://crates.io/crates/safe-transmute) - `safe-transmute` compares to `mem-markers` in much the same way as `bytemuck` and `zerocopy`. 