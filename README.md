# vsort-rs

Rust bindings for the Google Highway
[vectorized quicksort](https://github.com/google/highway/tree/master/hwy/contrib/sort).

This project is forked from [lincot/vqsort-rs](https://github.com/lincot/vqsort-rs). I've removed the need to install highway separately by configuring it to compile as a git submodule. New features include functions for partial sorting and selecting the nth element, as well as the introduction of the K32V32 and K64V64 types. Contributions are welcome!

## Example

### Sorting in Ascending Order
```rust
let mut numbers = [5, 3, 8, 0, -100];
vsort_rs::sort(&mut numbers);
assert_eq!(numbers, [-100, 0, 3, 5, 8]);
```

### Sorting in Descending Order
```rust
let mut numbers = [5, 3, 8, 0, -100];
vsort_rs::sort_descending(&mut numbers);
assert_eq!(numbers, [8, 5, 3, 0, -100]);
```

### Partial Sorting
```rust
let mut numbers = [5, 3, 8, 0, -100];
let k = 3;
vsort_rs::partial_sort(&mut numbers, k);
// The first 3 elements are the smallest in sorted order.
assert!(numbers[0] <= numbers[1] && numbers[1] <= numbers[2]);
```

### Selecting the nth Element
Same as https://doc.rust-lang.org/beta/std/primitive.slice.html#method.select_nth_unstable
```rust
let mut numbers = [5, 3, 8, 0, -100];
let k = 2;
vsort_rs::select_nth_unstable(&mut numbers, k);
```

### Sorting K32V32
```rust
let mut numbers = [
    vsort_rs::K32V32::new(5, 0),
    vsort_rs::K32V32::new(3, 1),
    vsort_rs::K32V32::new(8, 2),
    vsort_rs::K32V32::new(0, 3),
    vsort_rs::K32V32::new(100, 4),
];
vsort_rs::sort(&mut numbers);
```

## TODO
- Add back miri tests
- Add support u128
- Make different type as feature flags to reduce compile time and binary size