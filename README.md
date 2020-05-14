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