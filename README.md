# force_copy

**NOTE**: This is still WIP.

A Rust crate that allows making a non-`Copy` type into a `Copy` type.
Simply wrap your type in `ForceCopy` (e.g. `ForceCopy<MyType>`) - the resulting type implements `Copy`, and can be used in a `#[derive(Copy]` struct.

## Limitations

The wrapped type must not have a destructor - this is a language-level limitation which cannot be circumvented. 
To ignore the destructor of a type, wrap it in `ManuallyDrop` (e.g. `ForceCopy<ManuallyDrop<String>>`).
