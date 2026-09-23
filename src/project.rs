use crate::EncryptionAlgorithm;
use crate::types::PasskeyResidency;
use serde::{Deserialize, Serialize};
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

impl SecretpassProject {
    pub fn prf_salt(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.id, self.residency, self.algorithm, self.created_at
        )
    }
}
