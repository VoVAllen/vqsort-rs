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

pub fn partial_sort<T: VqsortItem>(data: &mut [T], k: usize) {
    VqsortItem::partial_sort(data, k);
}

pub fn partial_sort_descending<T: VqsortItem>(data: &mut [T], k: usize) {
    VqsortItem::partial_sort_descending(data, k);
}

pub fn select_nth_unstable<T: VqsortItem>(data: &mut [T], k: usize) {
    VqsortItem::select_nth_unstable(data, k);
}

pub fn select_nth_unstable_descending<T: VqsortItem>(data: &mut [T], k: usize) {
    VqsortItem::select_nth_unstable_descending(data, k);
}

/// A trait for types that can be sorted.
pub trait VqsortItem: Sized {
    fn sort(data: &mut [Self]);
    fn sort_descending(data: &mut [Self]);
    fn partial_sort(data: &mut [Self], k: usize);
    fn partial_sort_descending(data: &mut [Self], k: usize);
    fn select_nth_unstable(data: &mut [Self], k: usize);
    fn select_nth_unstable_descending(data: &mut [Self], k: usize);
}

macro_rules! vqsort_impl {
    ($($t:ty)*) => ($(
        paste::paste! {
            extern "C" {                
                fn [<vqsort_ $t>](data: *mut $t, len: usize);
                fn [<vqsort_ $t _descending>](data: *mut $t, len: usize);
                fn [<vqpartialsort_ $t>](data: *mut $t, len: usize, k: usize);
                fn [<vqpartialsort_ $t _descending>](data: *mut $t, len: usize, k: usize);
                fn [<vqselect_ $t>](data: *mut $t, len: usize, k: usize);
                fn [<vqselect_ $t _descending>](data: *mut $t, len: usize, k: usize);
            }

            impl VqsortItem for $t {
                #[inline]
                fn sort(data: &mut [Self]) {
                        unsafe { [<vqsort_ $t>](data.as_mut_ptr(), data.len()) };
                }

                #[inline]
                fn sort_descending(data: &mut [Self]) {
                        unsafe { [<vqsort_ $t _descending>](data.as_mut_ptr(), data.len()) };
                }

                #[inline]
                fn partial_sort(data: &mut [Self], k: usize) {
                        unsafe { [<vqpartialsort_ $t>](data.as_mut_ptr(), data.len(), k) };
                }

                #[inline]
                fn partial_sort_descending(data: &mut [Self], k: usize) {
                        unsafe { [<vqpartialsort_ $t _descending>](data.as_mut_ptr(), data.len(), k) };
                }

                #[inline]
                fn select_nth_unstable(data: &mut [Self], k: usize) {
                        unsafe { [<vqselect_ $t>](data.as_mut_ptr(), data.len(), k) };
                }

                #[inline]
                fn select_nth_unstable_descending(data: &mut [Self], k: usize) {
                        unsafe { [<vqselect_ $t _descending>](data.as_mut_ptr(), data.len(), k) };
                }
                
            }
        }
    )*)
}

vqsort_impl! { i16 u16 i32 u32 i64 u64 f32 f64 K32V32 K64V64}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct K32V32 {
    pub value: u32,
    pub key: u32,
}


#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct K64V64 {
    pub value: u64,
    pub key: u64,
}