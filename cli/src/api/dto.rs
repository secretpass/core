use secretpass_core::{
    SecretpassEnvironment, SecretpassProject, SecretpassUser,
    StoredPublicKey,
};
use serde::Deserialize;

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
