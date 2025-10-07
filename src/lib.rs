use std::mem::{self, MaybeUninit};

#[inline]
pub fn box_into_inner<T>(value: Box<T>) -> T {
    // Safety:
    // - `T` has valid `MaybeUninit<T>` bytes
    // - `Box<MaybeUninit<..>>` will deallocate memory without dropping the value of `T`
    let boxed_uninit = unsafe { mem::transmute::<Box<T>, Box<MaybeUninit<T>>>(value) };

    // Safety: the value was valid
    unsafe { boxed_uninit.assume_init_read() }
}

pub trait BoxIntoInner {
    type Inner;

    fn into_inner(self) -> Self::Inner;
}

impl<T> BoxIntoInner for Box<T> {
    type Inner = T;

    #[inline]
    fn into_inner(self) -> Self::Inner {
        box_into_inner(self)
    }
}
