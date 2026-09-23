use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen()]
#[derive(Clone, Serialize, Deserialize)]
pub enum PublicKeyType {
    User,
    Machine,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Serialize, Deserialize)]
pub struct StoredPublicKey {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: PublicKeyType, // user, machine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passkey: Option<Passkey>, // Only stored for user keys
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Passkey {
    pub user_id: String,
    pub cred_id: String,
    pub public_key: String, // Base64url-encoded COSE key
    pub created_at: i64,
    pub last_used_at: i64,
}
