use crate::passkeys::types::PrfResults;
use secretpass_core::{
    EncryptionAlgorithm, Passkey, PrivateKey, PrivateKeySeed, PublicKeyType, StoredPublicKey,
};
use std::collections::HashMap;

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

    pub fn decrypt(&self, cipher: &str) -> anyhow::Result<Vec<u8>> {
        self.private_key.decrypt(cipher)
    }

    pub fn decrypt_package(
        &self,
        secrets_package: &HashMap<String, String>,
    ) -> anyhow::Result<HashMap<String, Vec<u8>>> {
        self.private_key.decrypt_package(secrets_package)
    }

    pub fn build_public_key(&self, name: String, passkey: &Passkey) -> StoredPublicKey {
        let public_key = self.private_key.public_key();

        StoredPublicKey {
            id: public_key.calculate_id(PublicKeyType::User),
            name,
            type_: PublicKeyType::User,
            ecc: public_key.ecc(),
            kem: public_key.kem(),
            passkey: Some(passkey.clone()),
        }
    }
}
