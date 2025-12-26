use std::hint::black_box;

#[inline(never)]
pub fn int_kernel(mut x: u32) -> u32 {
    for _ in 0..1000 {
        x = x
            .wrapping_mul(1664525)
            .wrapping_add(1013904223);
        x ^= x.rotate_left(13);
        x = black_box(x);
    }
    x
}
