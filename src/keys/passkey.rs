pub(crate) use crate::enums::PublicKeyType;
use crate::utils::bin_decode;
use crate::{PublicKey, PublicKeyDto};
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

impl StoredPublicKey {
    fn public_key(&self) -> anyhow::Result<PublicKey> {
        let dto = PublicKeyDto {
            kem: self.kem.clone(),
            ecc: self.ecc.clone(),
        };
        PublicKey::from_dto(dto)
    }
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

    pub fn id_bytes(&self) -> anyhow::Result<Vec<u8>> {
        bin_decode(&self.id)
    }

    pub fn public_key_bytes(&self) -> anyhow::Result<Vec<u8>> {
        bin_decode(&self.public_key)
    }
}
