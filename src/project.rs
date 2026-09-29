use crate::enums::{EncryptionAlgorithm, PasskeyResidency};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct SecretpassProject {
    pub id: String, // Fixed - can't be changed
    pub name: String,
    pub description: String,
    pub residency: PasskeyResidency,    // Fixed - can't be changed
    pub algorithm: EncryptionAlgorithm, // Fixed - can't be changed
    pub created_at: String,             // Fixed - can't be changed
}

impl Default for SecretpassProject {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "".to_string(),
            description: "".to_string(),
            residency: PasskeyResidency::SyncedAllowed,
            algorithm: EncryptionAlgorithm::ECC,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl SecretpassProject {
    pub fn prf_salt(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.id, self.residency, self.algorithm, self.created_at
        )
    }
}
