//! Rust bindings for the Google Highway
//! [vectorized quicksort](https://github.com/google/highway/tree/master/hwy/contrib/sort).

#![no_std]

/// Sorts `data` in ascending order.
pub fn sort<T: VqsortItem>(data: &mut [T]) {
    VqsortItem::sort(data);
}

/// Sorts `data` in descending order.
pub fn sort_descending<T: VqsortItem>(data: &mut [T]) {
    VqsortItem::sort_descending(data);
}

/// A trait for types that can be sorted.
pub trait VqsortItem: Sized {
    fn sort(data: &mut [Self]);
    fn sort_descending(data: &mut [Self]);
}

macro_rules! vqsort_impl {
    ($($t:ty)*) => ($(
        paste::paste! {
            extern "C" {
                fn [<vqsort_ $t>](data: *mut $t, len: usize);
                fn [<vqsort_ $t _descending>](data: *mut $t, len: usize);
            }

            impl VqsortItem for $t {
                #[inline]
                fn sort(data: &mut [Self]) {
                    if cfg!(miri) {
                        data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                    } else {
                        unsafe { [<vqsort_ $t>](data.as_mut_ptr(), data.len()) };
                    }
                }

                #[inline]
                fn sort_descending(data: &mut [Self]) {
                    if cfg!(miri) {
                        data.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
                    } else {
                        unsafe { [<vqsort_ $t _descending>](data.as_mut_ptr(), data.len()) };
                    }
                }
            }
        }
    )*)
}

vqsort_impl! { i16 u16 i32 u32 i64 u64 f32 f64 }

macro_rules! vqsort_size_impl {
    ($($size:expr => $t:ty,)*) => ($(
        paste::paste! {
            #[cfg(target_pointer_width = "" $size)]
            #[inline]
            fn sort(data: &mut [Self]) {
                if cfg!(miri) {
                    data.sort_unstable();
                } else {
                    unsafe { [<vqsort_ $t>](data.as_mut_ptr().cast(), data.len()) };
                }
            }

            #[cfg(target_pointer_width = "" $size)]
            #[inline]
            fn sort_descending(data: &mut [Self]) {
                if cfg!(miri) {
                    data.sort_unstable_by_key(|&x| core::cmp::Reverse(x));
                } else {
                    unsafe { [<vqsort_ $t _descending>](data.as_mut_ptr().cast(), data.len()) };
                }
            }
        }
    )*)
}

impl VqsortItem for isize {
    vqsort_size_impl! {
        16 => i16,
        32 => i32,
        64 => i64,
    }
}

impl VqsortItem for usize {
    vqsort_size_impl! {
        16 => u16,
        32 => u32,
        64 => u64,
    }
}

// highway uses a 16-bytes aligned uint128_t.
// Rust has u128 aligned the same way since 1.77:
// [#116672](https://github.com/rust-lang/rust/pull/116672)
#[rustversion::since(1.77)]
#[allow(improper_ctypes)]
extern "C" {
    fn vqsort_u128(data: *mut u128, len: usize);
    fn vqsort_u128_descending(data: *mut u128, len: usize);
}

#[rustversion::since(1.77)]
impl VqsortItem for u128 {
    #[inline]
    fn sort(data: &mut [Self]) {
        assert_eq!(core::mem::align_of::<u128>(), 16);
        if cfg!(miri) {
            data.sort_unstable();
        } else {
            unsafe { vqsort_u128(data.as_mut_ptr(), data.len()) };
        }
    }

    #[inline]
    fn sort_descending(data: &mut [Self]) {
        assert_eq!(core::mem::align_of::<u128>(), 16);
        if cfg!(miri) {
            data.sort_unstable_by_key(|&x| core::cmp::Reverse(x));
        } else {
            unsafe { vqsort_u128_descending(data.as_mut_ptr(), data.len()) };
        }
    }
}
