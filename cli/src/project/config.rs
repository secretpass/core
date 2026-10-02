use crate::project::directory::get_working_directory;
use crate::project::secret::SecretValue;
use secretpass_core::{
    SecretManagerConfig, SecretManagerConfigLock, SecretpassUser, StoredPublicKey,
    StoredPublicKeysLock,
};
use std::collections::HashMap;

use secretpass_core::utils::bin_encode;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct BackendManager {
    pub config: SecretManagerConfig,
    pub config_lock: SecretManagerConfigLock,
    pub public_keys: Vec<StoredPublicKey>,
    pub public_keys_lock: StoredPublicKeysLock,
}

impl BackendManager {
    pub fn load() -> anyhow::Result<Option<Self>> {
        let dir = get_working_directory();

        // Check if the project exists
        if !dir.project_dir().is_dir() || !dir.config_path().is_file() {
            // This a new project or cloud setup
            return Ok(None);
        }

        let config = yaml_serde::from_slice::<SecretManagerConfig>(dir.read_config()?.as_bytes())
            .map_err(|e| {
            anyhow::anyhow!("Error parsing config file .spass/config.yaml: {}", e)
        })?;

        let config_lock =
            yaml_serde::from_slice::<SecretManagerConfigLock>(dir.read_config_lock()?.as_bytes())
                .map_err(|e| {
                anyhow::anyhow!("Error parsing config file .spass/config.yaml.lock: {}", e)
            })?;

        let public_keys =
            yaml_serde::from_slice::<Vec<StoredPublicKey>>(dir.read_public_keys()?.as_bytes())
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Error parsing public keys file .spass/public-keys.json: {}",
                        e
                    )
                })?;

        let public_keys_lock =
            yaml_serde::from_slice::<StoredPublicKeysLock>(dir.read_public_keys_lock()?.as_bytes())
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Error parsing public keys file .spass/public-keys.json.lock: {}",
                        e
                    )
                })?;

        Ok(Some(Self {
            config,
            config_lock,
            public_keys,
            public_keys_lock,
        }))
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<SecretpassUser> {
        let user = self.config.users.iter().find(|u| u.username == username);
        user.cloned()
    }

    pub fn get_user_lock_package(
        &self,
        user: &SecretpassUser,
    ) -> anyhow::Result<(
        Vec<StoredPublicKey>,
        SecretManagerConfigLock,
        StoredPublicKeysLock,
    )> {
        let mut public_keys: Vec<StoredPublicKey> = vec![];
        let mut config_lock: SecretManagerConfigLock = Default::default();
        let mut public_keys_lock: StoredPublicKeysLock = Default::default();

        for pk in self.public_keys.iter() {
            if let Some(passkey) = &pk.passkey
                && passkey.username == user.username
                && passkey.user_id == user.id
            {
                public_keys.push(pk.clone());
                config_lock.insert(
                    pk.id.clone(),
                    self.config_lock
                        .get(&pk.id)
                        .ok_or_else(|| {
                            anyhow::format_err!("Config lock not found for key ID: {}", pk.id)
                        })?
                        .clone(),
                );
                public_keys_lock.insert(
                    pk.id.clone(),
                    self.public_keys_lock
                        .get(&pk.id)
                        .ok_or_else(|| {
                            anyhow::format_err!("Public key lock not found for key ID: {}", pk.id)
                        })?
                        .clone(),
                );
            }
        }

        Ok((public_keys, config_lock, public_keys_lock))
    }

    fn add_user(
        &mut self,
        user: SecretpassUser,
        public_key: StoredPublicKey,
        secrets: Vec<SecretValue>,
    ) {
        self.config.users.push(user);
        self.add_public_key(public_key, secrets);
    }

    fn add_public_key(&mut self, key: StoredPublicKey, secrets: Vec<SecretValue>) {
        // Save the secrets
        self.public_keys.push(key);

        // TODO: Add logic for writing secret values
    }

    fn read_secrets_value(path: &PathBuf) -> anyhow::Result<HashMap<String, String>> {
        if !path.exists() {
            return Ok(HashMap::new());
        }

        if !path.is_file() {
            return Err(anyhow::format_err!(
                "Path({}) is not a file",
                path.display()
            ));
        }

        let content = fs::read_to_string(path)?;
        let content: HashMap<String, String> = serde_json::from_str(&content)?;

        Ok(content)
    }

    fn add_key_secrets(key: &StoredPublicKey, secrets: Vec<SecretValue>) -> anyhow::Result<()> {
        let dir = get_working_directory();
        for secret in secrets {
            let secret_path =
                dir.secrets_path(secret.environment.clone(), secret.secret_name.clone());
            let mut content = Self::read_secrets_value(&secret_path)?;
            content.insert(key.id.clone(), secret.encrypted_value.clone());
            let json_content = serde_json::to_string_pretty(&content)?;
            fs::write(secret_path, json_content)?;
        }

        Ok(())
    }

    fn compile_lock_file(&self, content: String) -> anyhow::Result<HashMap<String, String>> {
        let mut lock_values: HashMap<String, String> = HashMap::new();

        for public_key in &self.public_keys {
            let encrypted_keys = public_key.encrypt(content.clone().into_bytes())?;
            lock_values.insert(public_key.id.clone(), bin_encode(&encrypted_keys));
        }

        Ok(lock_values)
    }

    pub(crate) fn write(&mut self) -> anyhow::Result<()> {
        let dir = get_working_directory();

        fs::create_dir_all(dir.project_dir().as_path())?;

        let config_content = serde_json::to_string_pretty(&self.config)?;
        self.config_lock = self.compile_lock_file(config_content.clone())?;

        let config_lock_content = serde_json::to_string_pretty(&self.config_lock)?;
        fs::write(dir.config_path(), config_content)?;
        fs::write(dir.config_lock_path(), config_lock_content)?;

        let public_keys_content = serde_json::to_string_pretty(&self.public_keys)?;
        self.public_keys_lock = self.compile_lock_file(public_keys_content.clone())?;

        let public_keys_lock_content = serde_json::to_string_pretty(&self.public_keys_lock)?;
        fs::write(dir.public_keys_path(), public_keys_content)?;
        fs::write(dir.public_keys_lock_path(), public_keys_lock_content)?;

        Ok(())
    }
}
