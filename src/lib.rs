//! This crate provides utilities for efficiently extracting the inner value from a `Box<T>`
//! without unnecessarily running the destructor of the contained value.
//! 
//! The main functionality is implemented using `std::mem::transmute` and `std::mem::MaybeUninit`
//! to safely convert a `Box<T>` directly into the inner value `T` without dropping it.
//! This can be useful in performance-critical code where you want to avoid the overhead
//! of running destructors unnecessarily.
//! 
//! ## Example
//! 
//! ```
//! use box_into_inner::IntoInner;
//! 
//! let boxed_value = Box::new("Hello, World!");
//! let inner_value = boxed_value.into_inner();
//!
//! assert_eq!(inner_value, "Hello, World!");
//! ```

use std::mem::{self, MaybeUninit};

/// Extracts the inner value from a `Box<T>`.
/// 
/// This function uses unsafe code to transmute a `Box<T>` to `Box<MaybeUninit<T>>` 
/// and then read the value without dropping it. This can be useful when you need 
/// to extract a value from a box without triggering its destructor.
/// 
/// # Example
/// 
/// ```
/// use box_into_inner::box_into_inner;
/// 
/// let boxed_value = Box::new(vec![1, 2, 3, 4, 5]);
/// let inner_value = box_into_inner(boxed_value);
/// assert_eq!(inner_value, vec![1, 2, 3, 4, 5]);
/// ```
#[inline]
pub fn box_into_inner<T>(value: Box<T>) -> T {
    // Safety:
    // - `T` has valid `MaybeUninit<T>` bytes
    // - `Box<MaybeUninit<..>>` will deallocate memory without dropping the value of `T`
    let boxed_uninit = unsafe { mem::transmute::<Box<T>, Box<MaybeUninit<T>>>(value) };

    // Safety: the value was valid
    unsafe { boxed_uninit.assume_init_read() }
}

/// A trait that provides a method to extract the inner value from a container
/// without running its destructor.
/// 
/// Currently implemented for `Box<T>`, allowing you to call `.into_inner()` 
/// directly on boxed values.
pub trait IntoInner {
    /// The inner type contained in the container.
    type Inner;

    /// Extracts the inner value from the container without running its destructor.
    fn into_inner(self) -> Self::Inner;
}

impl<T> IntoInner for Box<T> {
    type Inner = T;

    #[inline]
    fn into_inner(self) -> Self::Inner {
        box_into_inner(self)
    }
}
