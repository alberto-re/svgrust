use rand::Rng;

/// A seed value for deterministic pseudorandom number generators
#[derive(Clone)]
pub struct Seed {
    seed: u32,
}

impl Seed {
    /// Construct a new seed with a random value
    pub fn random() -> Self {
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

impl From<Seed> for u32 {
    fn from(val: Seed) -> Self {
        val.seed
    }
}

impl From<Seed> for u64 {
    fn from(val: Seed) -> Self {
        val.seed as u64
    }
}
