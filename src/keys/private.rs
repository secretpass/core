use crate::keys::derive::{
    EccPrivateKey, EccPublicKey, EncryptionAlgorithm, KemPrivateKey, PrivateKeySeed,
    derive_ecc_keys, derive_kem_keys,
};
use crate::keys::public::{EncryptedPackage, PublicKey};
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use anyhow::anyhow;
use fips203::ml_kem_768::CipherText;
use fips203::traits::{Decaps, SerDes};

#[derive(Clone)]
pub struct PrivateKey {
    ecc: Option<EccPrivateKey>,
    kem: Option<KemPrivateKey>,
    public_key: PublicKey,
}

impl PrivateKey {
    pub fn from_seed(seed: PrivateKeySeed) -> anyhow::Result<Self> {
        let (ecc_public_key, ecc_private_key) = if seed.algorithm == EncryptionAlgorithm::ECC
            || seed.algorithm == EncryptionAlgorithm::Hybrid
        {
            let (public_key, private_key) = derive_ecc_keys(&seed)?;
            (Some(public_key), Some(private_key))
        } else {
            (None, None)
        };

        let (kem_public_key, ken_private_key) = if seed.algorithm == EncryptionAlgorithm::KEM
            || seed.algorithm == EncryptionAlgorithm::Hybrid
        {
            let (public_key, private_key) = derive_kem_keys(&seed)?;
            (Some(public_key), Some(private_key))
        } else {
            (None, None)
        };

        let public_key = PublicKey::from_keys(kem_public_key, ecc_public_key);
        let private_key = Self {
            ecc: ecc_private_key,
            kem: ken_private_key,
            public_key,
        };

        Ok(private_key)
    }

    pub fn decrypt(&self, payload: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        let encrypted_package = rmp_serde::from_slice::<EncryptedPackage>(payload.as_slice())?;

        let ecc_shared_secret = self.get_ecc_aes_shared_secret(&encrypted_package)?;
        let kem_shared_secret = self.get_kem_aes_shared_secret(&encrypted_package)?;
        let mut decrypted_payload = encrypted_package.payload.clone();

        for shared_secret in [kem_shared_secret, ecc_shared_secret].into_iter().flatten() {
            decrypted_payload = Self::aes_decrypt_payload(
                encrypted_package.nonce,
                &shared_secret,
                decrypted_payload,
            )?
        }

        Ok(decrypted_payload)
    }

    fn aes_decrypt_payload(
        nonce: [u8; 12],
        shared_secret: &[u8; 32],
        payload: Vec<u8>,
    ) -> anyhow::Result<Vec<u8>> {
        let cipher = Aes256Gcm::new_from_slice(shared_secret)?;
        let nonce = Nonce::from(nonce);

        let decrypted_payload = cipher.decrypt(&nonce, payload.as_slice())?;

        Ok(decrypted_payload)
    }

    fn get_ecc_aes_shared_secret(
        &self,
        encrypted_package: &EncryptedPackage,
    ) -> anyhow::Result<Option<[u8; 32]>> {
        if self.ecc.is_none() && encrypted_package.ecc_public_key.is_none() {
            return Ok(None);
        }
        if self.ecc.is_none() || encrypted_package.ecc_public_key.is_none() {
            return Err(anyhow!("Invalid payload/private key"));
        }

        let secret_key = self.ecc.clone().unwrap();
        let encryptor_public_key = EccPublicKey::from(encrypted_package.ecc_public_key());

        let shared_secret = secret_key.diffie_hellman(&encryptor_public_key);

        Ok(Some(shared_secret.to_bytes()))
    }

    fn get_kem_aes_shared_secret(
        &self,
        encrypted_package: &EncryptedPackage,
    ) -> anyhow::Result<Option<[u8; 32]>> {
        if self.kem.is_none() && encrypted_package.kem_ciphertext.is_none() {
            return Ok(None);
        }
        if self.kem.is_none() || encrypted_package.kem_ciphertext.is_none() {
            return Err(anyhow!("Invalid payload/private key"));
        }

        let ciphertext = CipherText::try_from_bytes(encrypted_package.kem_ciphertext()).unwrap();

        let secret_key = self.kem.clone().unwrap();
        let shared_secret = secret_key.try_decaps(&ciphertext).unwrap();

        Ok(Some(shared_secret.into_bytes()))
    }

    pub fn public_key(&self) -> PublicKey {
        self.public_key.clone()
    }
}
