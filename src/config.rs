use crate::user::SecretpassUser;
use crate::{OperationMode, SecretpassEnvironment, SecretpassProject};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct SecretManagerConfig {
    pub mode: OperationMode,
    pub project: SecretpassProject,
    pub environments: Vec<SecretpassEnvironment>,
    pub users: Vec<SecretpassUser>,
}

pub type SecretManagerConfigLock = HashMap<String, String>;
