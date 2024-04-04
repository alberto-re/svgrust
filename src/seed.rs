use rand::Rng;
use std::convert::Into;

/// A seed value for deterministic pseudorandom number generators
#[derive(Clone)]
pub struct Seed {
    seed: u32,
}

impl Seed {
    /// Construct a new seed with a random value
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let seed = rng.gen();
        println!("Generated seed value {seed}");
        Self { seed }
    }

    /// Construct a new seed from a number
    pub fn from_number(seed: u32) -> Self {
        Self { seed }
    }
}

impl Into<u32> for Seed {
    fn into(self) -> u32 {
        self.seed
    }
}

impl Into<u64> for Seed {
    fn into(self) -> u64 {
        self.seed as u64
    }
}
