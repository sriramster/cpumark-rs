#[derive(Debug)]
pub struct Config {
    pub duration_secs: f64,
    pub mem_size: usize,
    pub seed: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            duration_secs: 5.0,
            mem_size: 32 * 1024 * 1024, // 32 MB
            seed: 0x1234_5678,
        }
    }
}
