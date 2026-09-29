use crate::login_user;
use crate::passkeys::types::PrfResults;
use secretpass_core::utils::sha256_digest;
use secretpass_core::{
    Passkey, PrivateKey, PrivateKeySeed, PublicKey, PublicKeyDto, PublicKeyType, SecretpassProject,
    StoredPublicKey,
};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Uint8Array;

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
    pub fn new(project: SecretpassProject, passkey: Passkey, prf_results: PrfResults) -> Self {
        let seed = PrivateKeySeed {
            algorithm: project.algorithm,
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
pub async fn authorize_and_decrypt(params: &str, payload: Uint8Array) -> Uint8Array {
    let session = login_user(params).await;
    let payload = payload.to_vec();

    let payload = session
        .private_key
        .decrypt(payload)
        .expect_throw("Error decrypting payload");
    Uint8Array::from(payload.as_slice())
}

#[wasm_bindgen]
pub fn encrypt_payload(payload: Uint8Array, public_keys: Vec<PublicKeyJson>) -> Vec<Uint8Array> {
    let payload = payload.to_vec();

    // TODO: Specify the public key that has an issue
    let results: Vec<Uint8Array> = public_keys
        .iter()
        .map(|pk| {
            let key = PublicKey::from_dto(pk.to_dto()).expect_throw("Error decoding public key");
            let encrypted_payload = key
                .encrypt(payload.clone())
                .expect_throw("Error encrypting payload");
            Uint8Array::from(encrypted_payload.as_slice())
        })
        .collect();

    results
}

#[wasm_bindgen]
pub async fn authorize_and_build_public_key(params: &str, name: String) -> String {
    let session = login_user(params).await;

    let public_key = session.private_key.public_key();
    let pk_dto = public_key.to_dto();

    let id_salt = format!(
        "{}-{}-{}",
        PublicKeyType::User,
        pk_dto.ecc.clone().unwrap_or(String::from("null")),
        pk_dto.kem.clone().unwrap_or(String::from("null"))
    );

    let id = sha256_digest(&id_salt);

    let public_key = StoredPublicKey {
        id,
        name,
        type_: PublicKeyType::User,
        ecc: pk_dto.ecc,
        kem: pk_dto.kem,
        passkey: Some(session.passkey.clone()),
    };

    serde_json::to_string(&public_key).expect_throw("Error serializing public key")
}
