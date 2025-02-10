# vqsort-rs

Rust bindings for the Google Highway
[vectorized quicksort](https://github.com/google/highway/tree/master/hwy/contrib/sort).

The vectorized quicksort sorting algorithm is very fast, as seen in a
[writeup](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/intel_avx512/text.md),
and outperforms the standard Rust `sort_unstable`. However,
it can only be used with primitive integers and floats.

## Installation 

Only use clang-15, clang-17 doesn't work on my machine.

Modify highway CMakeLists.txt to build faster
```
-set(HWY_ENABLE_EXAMPLES ON CACHE BOOL "Build examples")
+set(HWY_ENABLE_EXAMPLES OFF CACHE BOOL "Build examples")
-set(HWY_ENABLE_TESTS ON CACHE BOOL "Enable HWY tests")
+set(HWY_ENABLE_TESTS OFF CACHE BOOL "Enable HWY tests")
-option(HWY_FORCE_STATIC_LIBS "Ignore BUILD_SHARED_LIBS" OFF)
+option(HWY_FORCE_STATIC_LIBS "Ignore BUILD_SHARED_LIBS" ON)
```

And do `sudo make install` in the build directory

## Example

```rust
let mut numbers = [5, 3, 8, 0, -100];
vqsort_rs::safe_vq_partialsort_i32(&mut numbers, k);
```

## Miri

When testing with Miri, this crate resorts to `sort_unstable`,
because Miri doesn't support FFI.
