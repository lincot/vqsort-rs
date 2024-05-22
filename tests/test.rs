#![no_std]

use rand::Rng;
use rand_pcg::Pcg64Mcg;

macro_rules! gen_tests {
    ($($t:ty)*) => ($(
        paste::paste! {
            #[test]
            fn [<test_vqsort_ $t>]() {
                let mut arr: [$t; 4] = [5u8.into(), 3u8.into(), 8u8.into(), 0u8.into()];
                vqsort_rs::sort(&mut arr);
                assert_eq!(arr, [0u8.into(), 3u8.into(), 5u8.into(), 8u8.into()]);

                let mut arr: [$t; 4] = [5u8.into(), 3u8.into(), 8u8.into(), 0u8.into()];
                vqsort_rs::sort_descending(&mut arr);
                assert_eq!(arr, [8u8.into(), 5u8.into(), 3u8.into(), 0u8.into()]);

                let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
                let mut buf: [$t; 1 << 12] = [0u8.into(); 1 << 12];
                for _ in 0..10 {
                    let len = rng.gen_range(0..buf.len());
                    let data = &mut buf[..len];

                    for x in data.iter_mut() {
                        *x = rng.gen();
                    }
                    vqsort_rs::sort(data);
                    assert!(is_sorted(data));

                    for x in data.iter_mut() {
                        *x = rng.gen();
                    }
                    vqsort_rs::sort_descending(data);
                    assert!(is_sorted_descending(data));
                }
            }
        }
    )*)
}

gen_tests! { i16 u16 i32 u32 i64 u64 isize usize f32 f64 }
#[rustversion::since(1.77)]
gen_tests! { u128 }

fn is_sorted<T: PartialOrd>(data: &[T]) -> bool {
    data.windows(2).all(|pair| pair[0] <= pair[1])
}

fn is_sorted_descending<T: PartialOrd>(data: &[T]) -> bool {
    data.windows(2).all(|pair| pair[0] >= pair[1])
}
