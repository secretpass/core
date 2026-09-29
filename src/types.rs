use crate::enums::UserLevel;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct SecretpassUser {
    pub id: String,
    pub username: String,
    pub name: String,
    pub level: UserLevel,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub access: Vec<String>,
    pub added_on: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct SecretpassEnvironment {
    pub name: String,
    pub description: String,
}
