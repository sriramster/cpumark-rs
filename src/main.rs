mod config;
mod score;
mod kernels;

use config::Config;
use score::Score;
use kernels::*;
use std::time::Instant;
use std::hint::black_box;

fn main() {
    let cfg = Config::default();

    println!("CpuMark starting...");
    println!("Config: {:?}", cfg);

    let mem_buf = mem::init_buffer(cfg.mem_size);
    let branch_data: Vec<u32> = (0..1_000_000).map(|i| i as u32 ^ 0xA5A5A5A5).collect();

    for _ in 0..10 {
        black_box(int::int_kernel(123));
        black_box(mem::mem_kernel(&mem_buf, 1000));
        black_box(branch::branch_kernel(&branch_data[..1000]));
    }

    for _ in 0..5 {
        let l = list::list_add(500);
        let l = list::list_bubble_sort(Some(l));
        let l = list::list_reverse(l);
        black_box(list::list_checksum(&l));
    }

    let start = Instant::now();
    let mut iterations = 0u64;
    let mut checksum = 0u64;

    while start.elapsed().as_secs_f64() < cfg.duration_secs {
        let x = int::int_kernel(iterations as u32);
        let y = mem::mem_kernel(&mem_buf, 100);
        let z = branch::branch_kernel(&branch_data[..1024]);

        let l = list::list_add(300);
        let l = list::list_bubble_sort(Some(l));
        let l = list::list_reverse(l);
        let lc = list::list_checksum(&l);

        checksum ^= x as u64;
        checksum ^= y as u64;
        checksum ^= z as u64;
        checksum ^= lc as u64;

        iterations += 1;
    }

    let elapsed = start.elapsed().as_secs_f64();

    let score = Score {
        iterations,
        seconds: elapsed,
        checksum,
    };

    println!("Iterations   : {}", score.iterations);
    println!("Time (s)     : {:.3}", score.seconds);
    println!("Checksum     : 0x{:x}", score.checksum);
    println!("CpuMark-rust Score: {:.2}", score.score());
}
