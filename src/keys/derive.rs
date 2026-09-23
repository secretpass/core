use anyhow::Ok;
use argon2::Argon2;
pub use fips203::ml_kem_768::{
    DecapsKey as KemPrivateKey, EncapsKey as KemPublicKey, KG as KemKeygen,
};
use fips203::traits::KeyGen;
use serde::{Deserialize, Serialize};
pub use x25519_dalek::{PublicKey as EccPublicKey, StaticSecret as EccPrivateKey};

#[derive(Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum EncryptionAlgorithm {
    ECC,    // X25519
    KEM,    // ML-KEM-768
    Hybrid, // Encrypts with both algorithms
}

#[derive(Clone, Deserialize)]
pub struct PrivateKeySeed {
    pub algorithm: EncryptionAlgorithm,
    pub first: [u8; 32],
    pub second: [u8; 32],
}

pub fn derive_ecc_keys(pk_seed: &PrivateKeySeed) -> anyhow::Result<(EccPublicKey, EccPrivateKey)> {
    let mut seed = [0u8; 32];

    Argon2::default().hash_password_into(&pk_seed.first, &pk_seed.second, &mut seed)?;

    let private_key = EccPrivateKey::from(seed);
    let public_key = EccPublicKey::from(&private_key);

    Ok((public_key, private_key))
}

pub fn derive_kem_keys(pk_seed: &PrivateKeySeed) -> anyhow::Result<(KemPublicKey, KemPrivateKey)> {
    let (public_key, private_key) = KemKeygen::keygen_from_seed(pk_seed.first, pk_seed.second);

    Ok((public_key, private_key))
}
