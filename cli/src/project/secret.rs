use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretDefinition {
    pub name: String,
    pub description: String,
    pub environment: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretValue {
    pub secret_name: String,
    pub environment: String,
    pub user_id: String,
    pub public_key_id: String,
    pub encrypted_value: String,
}

impl SecretValue {
    pub fn write() {}
}
