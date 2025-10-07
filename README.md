# Box Into Inner

This Rust crate provides utilities for efficiently extracting the inner value
from a `Box<T>` without unnecessarily running the destructor of the contained value.

The main functionality is implemented using `std::mem::transmute` and
`std::mem::MaybeUninit` to safely convert a `Box<T>` directly into the inner
value `T` without dropping it. This can be useful in performance-critical code
where you want to avoid the overhead of running destructors unnecessarily.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
box_into_inner = "0.1.0"
```

## Usage

### Using the Trait Method

```rust
use box_into_inner::BoxIntoInner;

fn main() {
    let boxed_value = Box::new("Hello, World!");
    let inner_value = boxed_value.into_inner();
    assert_eq!(inner_value, "Hello, World!");
}
```

### Using the Free Function

```rust
use box_into_inner::box_into_inner;

fn main() {
    let boxed_value = Box::new(42);
    let inner_value = box_into_inner(boxed_value);
    assert_eq!(inner_value, 42);
}
```

## API Documentation

### `box_into_inner<T>(value: Box<T>) -> T`

Extracts the inner value from a `Box<T>` without running its destructor.
This function uses unsafe code internally to transmute a `Box<T>` to
`Box<MaybeUninit<T>>` and then read the value without dropping it.

### `BoxIntoInner` Trait

A trait that provides a method to extract the inner value from a container without running its destructor.

- `type Inner`: The inner type contained in the container
- `fn into_inner(self) -> Self::Inner`: Extracts the inner value from the container

Currently implemented for `Box<T>`, allowing you to call `.into_inner()` directly on boxed values.

## Safety Considerations

The implementation uses `unsafe` code internally since it relies on
`mem::transmute` to convert between `Box<T>` and `Box<MaybeUninit<T>>`.
However, these operations are safe because:

- `T` has valid `MaybeUninit<T>` bytes (any valid value of `T` is also a valid value of `MaybeUninit<T>`)
- `Box<MaybeUninit<..>>` will deallocate memory without dropping the value of `T`

The exposed API is completely safe to use.

## Use Cases

This crate is useful when you need to extract a value from a `Box<T>` without
running its destructor. This can be important in scenarios where:

1. Performance is critical and you want to avoid destructor overhead
2. You're working with values that have expensive cleanup operations
3. You need to handle memory management in specific ways

## License

This project is licensed under the MIT License.
