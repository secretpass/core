use crate::enums::{EncryptionAlgorithm, PasskeyResidency};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[wasm_bindgen]
impl SecretpassProject {
    #[wasm_bindgen(constructor)]
    pub fn new_from_js(
        id: String,
        name: String,
        description: String,
        algorithm: &str,
        residency: &str,
        created_at: String,
    ) -> SecretpassProject {
        let algorithm = EncryptionAlgorithm::from_str(algorithm).unwrap();
        let residency = PasskeyResidency::from_str(residency).unwrap();

        SecretpassProject {
            id,
            name,
            description,
            residency,
            algorithm,
            created_at,
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
