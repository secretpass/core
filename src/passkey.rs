pub(crate) use crate::enums::PublicKeyType;
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct Passkey {
    pub id: String,
    pub public_key: String, // Base64url-encoded COSE key
    pub user_id: String,
    pub user_name: String,
    pub created_at: String,
}

impl Passkey {
    pub fn prf_salt(&self) -> String {
        format!(
            "{}-{}-{}-{}-{}",
            self.id, self.user_id, self.user_name, self.public_key, self.created_at
        )
    }

    pub fn id_bytes(&self) -> Vec<u8> {
        BASE64_URL_SAFE_NO_PAD.decode(&self.id).unwrap()
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        BASE64_URL_SAFE_NO_PAD.decode(&self.public_key).unwrap()
    }
}
