use crate::passkeys::types::PrfResults;
use secretpass_core::utils::sha256_digest;
use secretpass_core::{
    EncryptionAlgorithm, Passkey, PrivateKey, PrivateKeySeed, PublicKeyType, StoredPublicKey,
};

#[derive(Clone)]
pub struct SecureSession {
    private_key: PrivateKey,
}

impl SecureSession {
    pub fn new(algorithm: EncryptionAlgorithm, prf_results: PrfResults) -> anyhow::Result<Self> {
        let seed = PrivateKeySeed {
            algorithm,
            first: prf_results.first,
            second: prf_results.second,
        };

        let private_key = PrivateKey::from_seed(seed)?;

        Ok(Self { private_key })
    }

    pub fn decrypt(&self, cipher: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        self.private_key.decrypt(cipher)
    }

    pub fn build_public_key(&self, name: String, passkey: &Passkey) -> StoredPublicKey {
        let public_key = self.private_key.public_key();
        let ecc = public_key.ecc();
        let kem = public_key.kem();

        let id_salt = format!(
            "{}-{}-{}",
            PublicKeyType::User,
            ecc.clone().unwrap_or(String::from("null")),
            kem.clone().unwrap_or(String::from("null"))
        );

        let id = sha256_digest(&id_salt);

        StoredPublicKey {
            id,
            name,
            type_: PublicKeyType::User,
            ecc,
            kem,
            passkey: Some(passkey.clone()),
        }
    }
}
