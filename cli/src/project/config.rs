use crate::project::directory::get_working_directory;
use crate::project::secret::{SecretDefinition, SecretValue};
use secretpass_core::{
    OperationMode, SecretpassEnvironment, SecretpassProject, SecretpassUser, StoredPublicKey,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretManagerConfig {
    pub mode: OperationMode,
    pub project: SecretpassProject,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub environments: Vec<SecretpassEnvironment>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub users: Vec<SecretpassUser>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub secrets: Vec<SecretDefinition>,
    #[serde(skip, default)]
    pub public_keys: Vec<StoredPublicKey>,
}

impl SecretManagerConfig {
    pub fn load() -> Option<Self> {
        let dir = get_working_directory();

        // Check if the project exists
        if !dir.project_dir().is_dir() || !dir.config_path().is_file() {
            // This a new project or cloud setup
            return None;
        }

        let config_content =
            fs::read(dir.config_path()).expect("Error reading config file .spass/config.yaml");

        let mut stored_config =
            yaml_serde::from_slice::<SecretManagerConfig>(config_content.as_slice())
                .expect("Error parsing config file .spass/config.yaml");

        let public_keys_content = fs::read(dir.public_keys_path())
            .expect("Error reading public keys file .spass/public_keys.yaml");

        stored_config.public_keys =
            yaml_serde::from_slice::<Vec<StoredPublicKey>>(public_keys_content.as_slice())
                .expect("Error parsing public keys file .spass/public_keys.yaml");

        Some(stored_config)
    }

    fn add_user(
        &mut self,
        user: SecretpassUser,
        public_key: StoredPublicKey,
        secrets: Vec<SecretValue>,
    ) {
        self.users.push(user);
        self.add_public_key(public_key, secrets);
    }

    fn add_public_key(&mut self, key: StoredPublicKey, _secrets: Vec<SecretValue>) {
        // Save the secrets
        self.public_keys.push(key);

        // TODO: Add logic for writing secret values
    }

    pub(crate) fn write(&self) -> Result<(), Error> {
        let dir = get_working_directory();

        fs::create_dir(dir.project_dir().as_path())?;

        let config_content = yaml_serde::to_string(&self).expect("Error serializing config");
        fs::write(dir.config_path(), config_content)?;

        let public_keys_content =
            serde_json::to_string(&self.public_keys).expect("Error serializing public keys");
        fs::write(dir.public_keys_path(), public_keys_content)?;

        Ok(())
    }
}
