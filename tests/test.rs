#![no_std]

use rand::Rng;
use rand_pcg::Pcg64Mcg;
use vsort_rs::K32V32;

const BUF_SIZE: usize = if cfg!(miri) { 25 } else { 1 << 12 };

macro_rules! gen_tests {
    ($($t:ty)*) => ($(
        paste::paste! {
            #[test]
            #[allow(clippy::float_cmp)]
            fn [<test_vqsort_ $t>]() {
                let mut arr: [$t; 4] = [5u8.into(), 3u8.into(), 8u8.into(), 0u8.into()];
                vsort_rs::sort(&mut arr);
                assert_eq!(arr, [0u8.into(), 3u8.into(), 5u8.into(), 8u8.into()]);

                let mut arr: [$t; 4] = [5u8.into(), 3u8.into(), 8u8.into(), 0u8.into()];
                vsort_rs::sort_descending(&mut arr);
                assert_eq!(arr, [8u8.into(), 5u8.into(), 3u8.into(), 0u8.into()]);

                let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
                let mut buf_vqsort: [$t; BUF_SIZE] = [0u8.into(); BUF_SIZE];
                let mut buf_stdsort = buf_vqsort.clone();
                for _ in 0..10 {
                    let len = rng.gen_range(0..BUF_SIZE);
                    let data_vqsort = &mut buf_vqsort[..len];
                    let data_stdsort = &mut buf_stdsort[..len];

                    data_vqsort.iter_mut().for_each(|x| *x = rng.gen());
                    data_stdsort.copy_from_slice(data_vqsort);
                    data_stdsort.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                    vsort_rs::sort(data_vqsort);
                    assert_eq!(data_vqsort, data_stdsort);
                    assert!(is_sorted(data_vqsort));

                    data_vqsort.iter_mut().for_each(|x| *x = rng.gen());
                    data_stdsort.copy_from_slice(data_vqsort);
                    data_stdsort.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
                    vsort_rs::sort_descending(data_vqsort);
                    assert_eq!(data_vqsort, data_stdsort);
                    assert!(is_sorted_descending(data_vqsort));
                }
            }
        }
    )*)
}

gen_tests! { i16 u16 i32 u32 i64 u64 f32 f64 }

fn is_sorted<T: PartialOrd>(data: &[T]) -> bool {
    data.windows(2).all(|pair| pair[0] <= pair[1])
}
fn is_sorted_descending<T: PartialOrd>(data: &[T]) -> bool {
    data.windows(2).all(|pair| pair[0] >= pair[1])
}

// Test safe_vq_partialsort_k32v32 with random data.
#[test]
fn test_safe_vq_partialsort_k32v32_random() {
    const LEN: usize = 256;
    let mut rng = Pcg64Mcg::new(0xdead_beef_cafe_babe);
    let mut data = [K32V32 { key: 0, value: 0 }; LEN];

    // Initialize with random keys and arbitrary values.
    for item in &mut data {
        let key = rng.gen_range(0..1000);
        *item = K32V32 {
            key,
            value: key.wrapping_mul(10),
        };
    }
    let k = rng.gen_range(1..=LEN);
    vsort_rs::partial_sort(&mut data, k);

    // Copy and fully sort the array to determine the expected first k elements.
    let mut expected = data;
    expected.sort_by_key(|item| item.key);

    // Verify that the first k elements are the smallest ones in order.
    for i in 0..k {
        assert_eq!(
            data[i].key, expected[i].key,
            "Mismatch at index {} for k={}",
            i, k
        );
    }
    // Check that the first k elements are sorted in non-decreasing order.
    assert!(data[..k].windows(2).all(|pair| pair[0].key <= pair[1].key));
}
