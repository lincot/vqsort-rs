#![no_std]
#![allow(clippy::large_stack_arrays)]

use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64Mcg;
use vqsort_rs::{Key32Value32, Key64Value64};

const BUF_SIZE: usize = if cfg!(miri) { 25 } else { 1 << 12 };

trait FromKey {
    fn from_key(rng: &mut impl Rng, key: u8) -> Self;
}

macro_rules! impl_from_key {
    ($($t:ty)*) => ($(
        impl FromKey for $t {
            fn from_key(_rng: &mut impl Rng, key: u8) -> Self {
                key.into()
            }
        }
    )*)
}

impl_from_key! { i16 u16 i32 u32 i64 u64 isize usize f32 f64 u128 }

impl FromKey for Key64Value64 {
    fn from_key(rng: &mut impl Rng, key: u8) -> Self {
        Self {
            value: rng.gen(),
            key: key.into(),
        }
    }
}

impl FromKey for Key32Value32 {
    fn from_key(rng: &mut impl Rng, key: u8) -> Self {
        Self {
            value: rng.gen(),
            key: key.into(),
        }
    }
}

macro_rules! gen_tests {
    ($($t:ty)*) => ($(
        paste::paste! {
            #[test]
            #[allow(clippy::float_cmp)]
            fn [<test_vqsort_ $t:snake>]() {
                let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
                for _ in 0..10 {
                    let mut arr = [0, 3, 5, 8].map(|x| <$t>::from_key(&mut rng, x));
                    let mut arr_vqsort = arr.clone();
                    arr_vqsort.shuffle(&mut rng);
                    vqsort_rs::sort(&mut arr_vqsort);
                    assert_eq!(arr_vqsort, arr);

                    arr.reverse();
                    arr_vqsort.shuffle(&mut rng);
                    vqsort_rs::sort_descending(&mut arr_vqsort);
                    assert_eq!(arr_vqsort, arr);
                }

                let mut buf_vqsort: [$t; BUF_SIZE] = [Default::default(); BUF_SIZE];
                let mut buf_stdsort = buf_vqsort.clone();
                for _ in 0..10 {
                    let len = rng.gen_range(0..BUF_SIZE);
                    let data_vqsort = &mut buf_vqsort[..len];
                    let data_stdsort = &mut buf_stdsort[..len];

                    data_vqsort.iter_mut().for_each(|x| *x = rng.gen());
                    data_stdsort.copy_from_slice(data_vqsort);
                    data_stdsort.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                    vqsort_rs::sort(data_vqsort);
                    assert_eq!(data_vqsort, data_stdsort);
                    assert!(is_sorted(data_vqsort));

                    data_vqsort.iter_mut().for_each(|x| *x = rng.gen());
                    data_stdsort.copy_from_slice(data_vqsort);
                    data_stdsort.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
                    vqsort_rs::sort_descending(data_vqsort);
                    assert_eq!(data_vqsort, data_stdsort);
                    assert!(is_sorted_descending(data_vqsort));
                }
            }
        }
    )*)
}

gen_tests! { i16 u16 i32 u32 i64 u64 isize usize f32 f64 }
#[cfg(feature = "rand")]
gen_tests! { Key64Value64 Key32Value32 }
#[rustversion::since(1.77)]
gen_tests! { u128 }

fn is_sorted<T: PartialOrd>(data: &[T]) -> bool {
    data.windows(2).all(|pair| pair[0] <= pair[1])
}

fn is_sorted_descending<T: PartialOrd>(data: &[T]) -> bool {
    data.windows(2).all(|pair| pair[0] >= pair[1])
}
