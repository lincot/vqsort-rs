# vqsort-rs

Rust bindings for Google's Highway
[vectorized quicksort](https://github.com/google/highway/tree/master/hwy/contrib/sort).

The vectorized quicksort sorting algorithm is very fast, as demonstrated in a
[writeup](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/intel_avx512/text.md),
and outperforms the standard Rust `sort_unstable`. However, it only supports
primitive integer and floating-point types, as well as key-value tuples.

Supported types:

* `i16`, `u16`
* `i32`, `u32`
* `i64`, `u64`
* `isize`, `usize`
* `u128`
* `f32`, `f64`
* `Key64Value64`, `Key32Value32`

## Examples

```rust
let mut data = [5, 3, 8, 0, -100];
vqsort_rs::sort(&mut data);
assert_eq!(data, [-100, 0, 3, 5, 8]);

vqsort_rs::sort_descending(&mut data);
assert_eq!(data, [8, 5, 3, 0, -100]);
```

```rust
use vqsort_rs::Key32Value32; // or Key64Value64

let mut data = [
    Key32Value32 { value: 0, key: 5 },
    Key32Value32 { value: 1, key: 3 },
    Key32Value32 { value: 2, key: 8 },
    Key32Value32 { value: 3, key: 0 },
];
vqsort_rs::sort(&mut data);
assert_eq!(
    data,
    [
        Key32Value32 { value: 3, key: 0 },
        Key32Value32 { value: 1, key: 3 },
        Key32Value32 { value: 0, key: 5 },
        Key32Value32 { value: 2, key: 8 },
    ]
);

vqsort_rs::sort_descending(&mut data);
assert_eq!(
    data,
    [
        Key32Value32 { value: 2, key: 8 },
        Key32Value32 { value: 0, key: 5 },
        Key32Value32 { value: 1, key: 3 },
        Key32Value32 { value: 3, key: 0 },
    ]
);
```

## Miri

When testing under Miri, this crate falls back to `sort_unstable` because Miri
does not support FFI.
