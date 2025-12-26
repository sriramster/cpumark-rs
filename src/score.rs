#[derive(Debug)]
pub struct Score {
    pub iterations: u64,
    pub seconds: f64,
    pub checksum: u64,
}

impl Score {
    pub fn score(&self) -> f64 {
        self.iterations as f64 / self.seconds
    }
}
