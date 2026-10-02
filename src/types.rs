use crate::{
    SecretManagerConfigLock, SecretpassProject, SecretpassUser, StoredPublicKey,
    StoredPublicKeysLock,
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
pub struct SecretDefinition {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
pub struct SecretpassEnvironment {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub secrets: Vec<SecretDefinition>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EncryptedPackage {
    pub ecc_public_key: Option<Vec<u8>>,
    pub kem_ciphertext: Option<Vec<u8>>,
    pub nonce: [u8; 12],
    pub payload: Vec<u8>,
}

impl EncryptedPackage {
    pub fn ecc_public_key(&self) -> [u8; 32] {
        let key_bytes: [u8; 32] = self
            .ecc_public_key
            .clone()
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap();
        key_bytes
    }

    pub fn kem_ciphertext(&self) -> [u8; 1088] {
        let key_bytes: [u8; 1088] = self
            .kem_ciphertext
            .clone()
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap();
        key_bytes
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct UserLockPackage {
    pub user: SecretpassUser,
    pub project: SecretpassProject,
    pub public_keys: Vec<StoredPublicKey>,
    pub config_lock: SecretManagerConfigLock,
    pub public_keys_lock: StoredPublicKeysLock,
}
