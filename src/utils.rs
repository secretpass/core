use hex;
use rand::SeedableRng;
use rand::rngs::ChaCha12Rng;
use sha2::{Digest, Sha256};
use std::sync::{Mutex, OnceLock};

static RNG: OnceLock<Mutex<ChaCha12Rng>> = OnceLock::new();

pub fn get_global_rng() -> &'static Mutex<ChaCha12Rng> {
    RNG.get_or_init(|| Mutex::new(ChaCha12Rng::from_seed(Default::default())))
}

pub fn sha256_digest(s: &str) -> String {
    let hash = Sha256::digest(s.as_bytes());

    hex::encode(hash.as_slice()).as_str().to_string()
}
