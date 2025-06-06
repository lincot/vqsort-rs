#include "hwy/contrib/sort/vqsort.h"
#include "hwy/base.h"

extern "C" {
void vqsort_i16(int16_t *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_i16_descending(int16_t *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}

void vqsort_u16(uint16_t *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_u16_descending(uint16_t *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}

void vqsort_i32(int32_t *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_i32_descending(int32_t *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}

void vqsort_u32(uint32_t *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_u32_descending(uint32_t *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}

void vqsort_i64(int64_t *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_i64_descending(int64_t *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}

void vqsort_u64(uint64_t *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_u64_descending(uint64_t *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}

void vqsort_u128(hwy::uint128_t *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_u128_descending(hwy::uint128_t *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}

void vqsort_f32(float *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_f32_descending(float *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}

void vqsort_f64(double *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_f64_descending(double *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}

void vqsort_k64v64(hwy::K64V64 *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_k64v64_descending(hwy::K64V64 *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}

void vqsort_k32v32(hwy::K32V32 *data, size_t len) {
  VQSort(data, len, hwy::SortAscending{});
}

void vqsort_k32v32_descending(hwy::K32V32 *data, size_t len) {
  VQSort(data, len, hwy::SortDescending{});
}
}
