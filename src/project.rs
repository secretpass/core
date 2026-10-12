use crate::enums::{EncryptionAlgorithm, PasskeyResidency};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
pub struct SecretpassProject {
    pub id: String, // Fixed - invalidates keys if changed
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub description: String,
    pub residency: PasskeyResidency, // Fixed - invalidates keys if changed
    pub algorithm: EncryptionAlgorithm, // Fixed - invalidates keys if changed
    pub created_at: String,          // Fixed - invalidates keys if changed
}

impl SecretpassProject {
    pub fn prf_salt(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.id, self.residency, self.algorithm, self.created_at
        )
    }
}
