use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::SeedableRng;
use rand::rngs::ChaCha12Rng;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use std::io::Cursor;
use std::sync::{Mutex, OnceLock};

static RNG: OnceLock<Mutex<ChaCha12Rng>> = OnceLock::new();

pub fn get_global_rng() -> &'static Mutex<ChaCha12Rng> {
    RNG.get_or_init(|| Mutex::new(ChaCha12Rng::from_seed(Default::default())))
}

pub fn sha256_digest(items: &[String]) -> String {
    let mut hash = Sha256::new();
    for value in items {
        hash.update(value.as_bytes());
    }

    let result = hash.finalize();

    hex::encode(result.as_slice())
}

pub fn bin_encode(payload: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(payload)
}

pub fn bin_decode(payload: &str) -> anyhow::Result<Vec<u8>> {
    match URL_SAFE_NO_PAD.decode(payload) {
        Ok(decoded) => Ok(decoded),
        Err(e) => Err(anyhow::anyhow!("Failed to decode base64: {}", e)),
    }
}

pub fn bin_serialize<T>(payload: &T) -> anyhow::Result<String>
where
    T: Serialize,
{
    // let serialized = postcard::to_allocvec_cobs(payload)?;
    let mut serialized = Vec::new();
    ciborium::ser::into_writer(payload, &mut serialized)?;
    Ok(bin_encode(&serialized))
}

pub fn bin_deserialize<T>(payload: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let decoded = bin_decode(payload)
        .map_err(|err| anyhow::format_err!("base64 content error: {:?}", err))?;

    let mut reader = Cursor::new(&decoded);
    let result: T = ciborium::de::from_reader(&mut reader)
        .map_err(|err| anyhow::format_err!("binary content error: {:?}", err))?;
    // let result: T = postcard::from_bytes_cobs(&mut decoded)
    //     .map_err(|err| anyhow::format_err!("binary content error: {:?}", err))?;
    Ok(result)
}
