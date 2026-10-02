use crate::UserLevel;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
pub struct SecretpassUser {
    pub id: String,       // Fixed - invalidates keys if changed
    pub username: String, // Fixed - invalidates keys if changed
    pub name: String,
    pub level: UserLevel, // Fixed - invalidates keys if changed
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub access: Vec<String>,
    pub created_at: String, // Fixed - invalidates keys if changed
}

impl SecretpassUser {
    pub fn prf_salt(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.id, self.username, self.level, self.created_at
        )
    }
}
