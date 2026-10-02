use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
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

    bin_encode(hash.as_slice())
}

pub fn bin_encode(payload: &[u8]) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(payload)
}

pub fn bin_decode(payload: &str) -> anyhow::Result<Vec<u8>> {
    match BASE64_URL_SAFE_NO_PAD.decode(payload) {
        Ok(decoded) => Ok(decoded),
        Err(e) => Err(anyhow::anyhow!("Failed to decode base64: {}", e)),
    }
}
