use secretpass_core::{
    SecretDataType, SecretManagerConfig, SecretpassEnvironment, SecretpassProject, SecretpassUser,
    StoredPublicKey,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct CreateProjectDTO {
    pub project: SecretpassProject,
    pub environments: Vec<SecretpassEnvironment>,
    pub user: SecretpassUser,
    pub public_key: StoredPublicKey,
}

#[derive(Debug, Deserialize)]
pub struct UserLockPackageRequestDTO {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectDTO {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub config: Option<SecretManagerConfig>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub public_keys: Option<Vec<StoredPublicKey>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrUpdateSecretDTO {
    pub description: String,
    pub data_type: SecretDataType,
    pub value: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrUpdateUserDTO {
    pub user: Option<SecretpassUser>, // None for machine keys
    pub public_key: StoredPublicKey,
    pub secret_values: HashMap<String, HashMap<String, String>>,
}
