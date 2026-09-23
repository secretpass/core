use crate::passkeys::types::PrfResults;
use secretpass_core::{
    EncryptionAlgorithm, Passkey, PrivateKey, PrivateKeySeed, PublicKey, PublicKeyDto,
    PublicKeyType, StoredPublicKey,
};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Uint8Array;

#[wasm_bindgen]
#[derive(Clone)]
pub struct SecureSession {
    passkey: Passkey,
    private_key: PrivateKey,
}

#[wasm_bindgen(getter_with_clone)]
pub struct PublicKeyJson {
    pub ecc: Option<String>,
    pub kem: Option<String>,
}

impl PublicKeyJson {
    pub fn to_dto(&self) -> PublicKeyDto {
        PublicKeyDto {
            ecc: self.ecc.clone(),
            kem: self.kem.clone(),
        }
    }
}

impl SecureSession {
    pub fn new(passkey: Passkey, algorithm: EncryptionAlgorithm, prf_results: PrfResults) -> Self {
        let seed = PrivateKeySeed {
            algorithm,
            first: prf_results.first,
            second: prf_results.second,
        };

        let private_key = PrivateKey::from_seed(seed).expect_throw("Error deriving private key");

        Self {
            passkey,
            private_key,
        }
    }
}

#[wasm_bindgen]
impl SecureSession {
    #[wasm_bindgen]
    pub fn decrypt(&self, payload: Uint8Array) -> Uint8Array {
        let payload = payload.to_vec();

        let payload = self
            .private_key
            .decrypt(payload)
            .expect_throw("Error decrypting payload");
        Uint8Array::from(payload.as_slice())
    }

    #[wasm_bindgen]
    pub fn encrypt(&self, payload: Uint8Array, public_keys: Vec<PublicKeyJson>) -> Vec<Uint8Array> {
        let payload = payload.to_vec();

        // TODO: Specify the public key that has an issue
        let results: Vec<Uint8Array> = public_keys
            .iter()
            .map(|pk| {
                let key =
                    PublicKey::from_dto(pk.to_dto()).expect_throw("Error decoding public key");
                let encrypted_payload = key
                    .encrypt(payload.clone())
                    .expect_throw("Error encrypting payload");
                Uint8Array::from(encrypted_payload.as_slice())
            })
            .collect();

        results
    }

    #[wasm_bindgen]
    pub fn build_public_key(&self, name: String) -> StoredPublicKey {
        let public_key = self.private_key.public_key();
        let pk_dto = public_key.to_dto();

        StoredPublicKey {
            name,
            type_: PublicKeyType::User,
            ecc: pk_dto.ecc,
            kem: pk_dto.kem,
            passkey: Some(self.passkey.clone()),
        }
    }
}
