use std::hint::black_box;

#[inline(never)]
pub fn branch_kernel(data: &[u32]) -> u32 {
    let mut acc = 0u32;
    for &x in data {
        if ((x ^ (x >> 1)) & 1) == 0 {
            acc = acc.wrapping_add(x);
        } else {
            acc = acc.wrapping_sub(x);
        }
        acc = black_box(acc);
    }
    acc
}
