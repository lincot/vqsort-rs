# vqsort-rs

Rust bindings for the Google Highway's
[vectorized quicksort](https://github.com/google/highway/tree/master/hwy/contrib/sort).

The sorting algorithm is very fast as seen in a
[research](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/intel_avx512/text.md)
and far outperforms the standard Rust sort_unstable. However,
it can only be used with integers and floats up to 64 bits.

## Miri

When testing with Miri, the crate resorts to sort_unstable,
since Miri doesn't support FFI.
