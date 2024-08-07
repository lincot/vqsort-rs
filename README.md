# vqsort-rs

Rust bindings for the Google Highway
[vectorized quicksort](https://github.com/google/highway/tree/master/hwy/contrib/sort).

The vectorized quicksort sorting algorithm is very fast, as seen in a
[writeup](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/intel_avx512/text.md),
and outperforms the standard Rust `sort_unstable`. However,
it can only be used with primitive integers and floats.

## Example

```rust
let mut data = [5, 3, 8, 0, -100];
vqsort_rs::sort(&mut data);
assert_eq!(data, [-100, 0, 3, 5, 8]);

vqsort_rs::sort_descending(&mut data);
assert_eq!(data, [8, 5, 3, 0, -100]);
```

## Miri

When testing with Miri, this crate resorts to `sort_unstable`,
because Miri doesn't support FFI.
