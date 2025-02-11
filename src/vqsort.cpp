#include "hwy/contrib/sort/vqsort.h"

#include "hwy/base.h"

extern "C" {
#define DEFINE_VQSORT(type, name)                                                         \
  void vqsort_##name(type* data, size_t len) { VQSort(data, len, hwy::SortAscending{}); } \
  void vqsort_##name##_descending(type* data, size_t len) {                               \
    VQSort(data, len, hwy::SortDescending{});                                             \
  }

#define DEFINE_VQPARTIALSORT(type, name)                                   \
  void vqpartialsort_##name(type* data, size_t n, size_t k) {              \
    VQPartialSort(data, n, k, hwy::SortAscending{});                       \
  }                                                                        \
  void vqpartialsort_##name##_descending(type* data, size_t n, size_t k) { \
    VQPartialSort(data, n, k, hwy::SortDescending{});                      \
  }

#define DEFINE_VQSELECT(type, name)                                   \
  void vqselect_##name(type* data, size_t n, size_t k) {              \
    VQSelect(data, n, k, hwy::SortAscending{});                       \
  }                                                                   \
  void vqselect_##name##_descending(type* data, size_t n, size_t k) { \
    VQSelect(data, n, k, hwy::SortDescending{});                      \
  }

#define DEFINE_VQ_ALL(type, name)  \
  DEFINE_VQSORT(type, name)        \
  DEFINE_VQPARTIALSORT(type, name) \
  DEFINE_VQSELECT(type, name)

DEFINE_VQ_ALL(int16_t, i16)
DEFINE_VQ_ALL(uint16_t, u16)
DEFINE_VQ_ALL(int32_t, i32)
DEFINE_VQ_ALL(uint32_t, u32)
DEFINE_VQ_ALL(int64_t, i64)
DEFINE_VQ_ALL(uint64_t, u64)
DEFINE_VQ_ALL(float, f32)
DEFINE_VQ_ALL(double, f64)
DEFINE_VQ_ALL(hwy::K32V32, K32V32)
DEFINE_VQ_ALL(hwy::K64V64, K64V64)
}
