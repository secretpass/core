use crate::user::SecretpassUser;
use crate::{OperationMode, SecretpassEnvironment, SecretpassProject};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretManagerConfig {
    pub mode: OperationMode,
    pub project: SecretpassProject,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub environments: Vec<SecretpassEnvironment>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub users: Vec<SecretpassUser>,
}

pub type SecretManagerConfigLock = HashMap<String, String>;
