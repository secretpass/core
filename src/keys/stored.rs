use crate::PublicKey;
pub(crate) use crate::enums::PublicKeyType;
use crate::utils::bin_decode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
pub struct StoredPublicKey {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: PublicKeyType, // user, machine
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub kem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ecc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub passkey: Option<Passkey>, // Only stored for user keys
}

impl StoredPublicKey {
    fn public_key(&self) -> anyhow::Result<PublicKey> {
        PublicKey::from_encoded(self.ecc.clone(), self.kem.clone())
    }

    pub fn encrypt(&self, payload: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        self.public_key()?.encrypt(payload)
    }
}

pub type StoredPublicKeysLock = HashMap<String, String>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
pub struct Passkey {
    pub id: String,
    pub public_key: String, // Base64url-encoded COSE key
    pub user_id: String,
    pub username: String,
    pub created_at: String,
}

impl Passkey {
    pub fn id_bytes(&self) -> anyhow::Result<Vec<u8>> {
        bin_decode(&self.id)
    }

    pub fn public_key_bytes(&self) -> anyhow::Result<Vec<u8>> {
        bin_decode(&self.public_key)
    }
}
