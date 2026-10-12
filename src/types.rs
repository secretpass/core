use crate::config::SecretManagerConfig;
use crate::enums::{CommandStatus, LogSource, SecretDataType};
use crate::keys::StoredPublicKey;
use crate::project::SecretpassProject;
use crate::user::SecretpassUser;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::{Tsify, declare};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
pub struct SecretDefinition {
    pub name: String,
    pub data_type: SecretDataType,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Tsify)]
pub struct SecretpassEnvironment {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub secrets: Vec<SecretDefinition>,
}

// No keys for smaller payloads
#[derive(Clone, Serialize, Deserialize)]
pub struct EncryptedPackage(
    /** ECC public key bytes */ pub Option<Vec<u8>>,
    /** KEM public key bytes */ pub Option<Vec<u8>>,
    /** Nonce Bytes */ pub [u8; 12],
    /** Encrypted payload */ pub Vec<u8>,
);

impl EncryptedPackage {
    pub fn ecc_public_key(&self) -> [u8; 32] {
        let key_bytes: [u8; 32] = self.0.clone().unwrap().as_slice().try_into().unwrap();
        key_bytes
    }

    pub fn kem_ciphertext(&self) -> [u8; 1088] {
        let key_bytes: [u8; 1088] = self.1.clone().unwrap().as_slice().try_into().unwrap();
        key_bytes
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct LoginParams {
    pub user: SecretpassUser,
    pub project: SecretpassProject,
    pub public_keys: Vec<StoredPublicKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct EnvironmentLockPackage {
    pub public_keys: Vec<StoredPublicKey>,
    pub encrypted_secrets: HashMap<String, String>,
}

#[declare]
pub type MultiEnvironmentLockPackage = HashMap<String, EnvironmentLockPackage>;

#[derive(Debug, Clone, Serialize, Tsify)]
pub struct LogItem(pub usize, pub String, pub LogSource, pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct CommandInstanceDetails {
    pub id: String,
    pub command: String,
    pub workdir: String,
    pub status: CommandStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tsify)]
pub struct UserApplicationState {
    pub directory: String,
    pub config: SecretManagerConfig,
    pub user: SecretpassUser,
    pub public_key: StoredPublicKey,
    pub public_keys: Vec<StoredPublicKey>,
    pub lock_package: MultiEnvironmentLockPackage,
}
