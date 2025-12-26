use std::hint::black_box;

#[inline(never)]
pub fn mem_kernel(buf: &[usize], iters: usize) -> usize {
    let mut idx = 0usize;
    for _ in 0..iters {
        idx = buf[idx];
        idx = black_box(idx);
    }
    idx
}

pub fn init_buffer(size: usize) -> Vec<usize> {
    let len = size / std::mem::size_of::<usize>();
    let mut buf = vec![0; len];

    for i in 0..len {
        buf[i] = (i * 17 + 13) % len; // pseudo-random cycle
    }
    buf
}
