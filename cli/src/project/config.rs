use crate::project::directory::get_working_directory;
use secretpass_core::{
    SecretDataType, SecretDefinition, SecretManagerConfig, SecretManagerConfigLock,
    SecretpassEnvironment, SecretpassUser, StoredPublicKey, StoredPublicKeysLock,
};
use std::collections::HashMap;

use secretpass_core::utils::bin_encode;
use std::fs;

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

    pub fn get_environment(&self, env_name: &str) -> Option<SecretpassEnvironment> {
        self.config
            .environments
            .iter()
            .find(|env| env.name == env_name)
            .cloned()
    }

    pub fn get_environment_mut(&mut self, env_name: &str) -> Option<&mut SecretpassEnvironment> {
        self.config
            .environments
            .iter_mut()
            .find(|env| env.name == env_name)
    }

    pub fn add_or_update_secret_definition(
        &mut self,
        env_name: &str,
        secret_name: &str,
        data_type: SecretDataType,
        description: String,
    ) -> Option<SecretDefinition> {
        let environment = self.get_environment_mut(env_name)?;

        let definition = environment
            .secrets
            .iter_mut()
            .find(|def| def.name == secret_name);
        if let Some(definition) = definition {
            definition.data_type = data_type;
            definition.description = description;
            Some(definition.clone())
        } else {
            let secret_definition = SecretDefinition {
                name: secret_name.to_string(),
                data_type,
                description,
            };
            environment.secrets.push(secret_definition.clone());
            Some(secret_definition)
        }
    }

    pub fn delete_secret_definition(&mut self, env_name: &str, secret_name: &str) {
        let environment = self.get_environment_mut(env_name);
        if let Some(environment) = environment {
            environment.secrets.retain(|def| def.name != secret_name);
        }
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

    pub fn get_public_keys_for_env(&self, env_name: String) -> Vec<StoredPublicKey> {
        let mut public_keys: Vec<StoredPublicKey> = Vec::new();
        for user in self.config.users.iter() {
            if !user.has_access(env_name.clone()) {
                continue;
            }
            for pk in self.public_keys.iter() {
                if let Some(passkey) = &pk.passkey
                    && passkey.user_id == user.id
                {
                    public_keys.push(pk.clone());
                }
            }
        }

        // TODO: Add logic for machine keys

        public_keys
    }

    pub fn read_secrets_value(
        env_name: &str,
        secret_name: &str,
    ) -> anyhow::Result<Option<HashMap<String, String>>> {
        let dir = get_working_directory();

        let path = dir.secrets_path(env_name, secret_name);
        if !path.exists() {
            return Ok(None);
        }

        if !path.is_file() {
            return Err(anyhow::format_err!(
                "Path({}) is not a file",
                path.display()
            ));
        }

        let content = fs::read_to_string(path)?;
        let content: HashMap<String, String> = serde_json::from_str(&content)?;

        Ok(Some(content))
    }

    pub fn write_secret_value(
        env_name: &str,
        secret_name: &str,
        value: HashMap<String, String>,
    ) -> anyhow::Result<()> {
        let dir = get_working_directory();
        let path = dir.secrets_path(env_name, secret_name);

        fs::create_dir_all(path.parent().unwrap())?;

        let content = serde_json::to_string_pretty(&value)?;
        fs::write(path, content)?;

        Ok(())
    }

    pub fn delete_secret(env_name: &str, secret_name: &str) -> anyhow::Result<()> {
        let dir = get_working_directory();
        let path = dir.secrets_path(env_name, secret_name);

        fs::remove_file(path)?;
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
