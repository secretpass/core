use crate::project::directory::{WorkingDirectory, get_working_directory};
use secretpass_core::{
    EnvironmentLockPackage, MultiEnvironmentLockPackage, PublicKeyType, SecretDataType,
    SecretDefinition, SecretManagerConfig, SecretManagerConfigLock, SecretpassEnvironment,
    SecretpassUser, StoredPublicKey, StoredPublicKeysLock, UserApplicationState, UserLevel,
};
use std::collections::HashMap;

use crate::project::environment::get_environment_encrypted_secrets;
use axum::http::StatusCode;
use std::fs;

#[derive(Debug, Clone)]
pub struct BackendManager {
    pub dir: WorkingDirectory,
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
            dir,
            config,
            config_lock,
            public_keys,
            public_keys_lock,
        }))
    }

    pub fn get_user_by_id(&self, user_id: &str) -> Option<SecretpassUser> {
        let user = self.config.users.iter().find(|u| u.id == user_id);
        user.cloned()
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<SecretpassUser> {
        for user in self.config.users.iter() {
            if let Some(current_username) = &user.username
                && current_username == username
            {
                return Some(user.clone());
            }
        }
        None
    }

    pub fn get_public_key(&self, key_id: &str) -> Option<StoredPublicKey> {
        self.public_keys
            .iter()
            .find(|key| key.id == key_id)
            .cloned()
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

    pub fn get_user_application_state(
        &self,
        key_id: &str,
    ) -> Result<UserApplicationState, (StatusCode, String)> {
        let public_key = self
            .get_public_key(key_id)
            .ok_or_else(|| (StatusCode::NOT_FOUND, "Public key not found".to_string()))?;
        let passkey = public_key
            .passkey
            .as_ref()
            .ok_or_else(|| (StatusCode::NOT_FOUND, "Passkey not found".to_string()))?;
        let user = self
            .get_user_by_id(&passkey.user_id)
            .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

        let environments = match user.level {
            UserLevel::Admin => self
                .config
                .environments
                .iter()
                .map(|env| env.name.clone())
                .collect(),
            _ => user.access.clone(),
        };

        let mut lock_package = MultiEnvironmentLockPackage::new();
        for env_name in environments {
            let environment = self.get_environment(&env_name).ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    format!("Environment {env_name} not found"),
                )
            })?;

            let public_keys = self.get_public_keys_for_env(env_name.clone());
            let encrypted_secrets = get_environment_encrypted_secrets(&environment, &public_key)?;

            lock_package.insert(
                env_name.clone(),
                EnvironmentLockPackage {
                    public_keys,
                    encrypted_secrets,
                },
            );
        }

        Ok(UserApplicationState {
            directory: self.dir.short_path(),
            config: self.config.clone(),
            user,
            public_key,
            public_keys: self.public_keys.clone(),
            lock_package,
        })
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

    pub fn get_user_public_keys(&self, user: &SecretpassUser) -> Option<Vec<StoredPublicKey>> {
        let mut public_keys: Vec<StoredPublicKey> = vec![];

        for pk in self.public_keys.iter() {
            let is_a_match = if let Some(passkey) = &pk.passkey {
                passkey.user_id == user.id
            } else {
                pk.type_ == PublicKeyType::Machine
                    && user.level == UserLevel::Machine
                    && pk.id == user.id
            };

            if is_a_match {
                public_keys.push(pk.clone());
            }
        }

        if public_keys.is_empty() {
            return None;
        }

        Some(public_keys)
    }

    pub fn get_public_keys_for_env(&self, env_name: String) -> Vec<StoredPublicKey> {
        let mut public_keys: Vec<StoredPublicKey> = Vec::new();
        for user in self.config.users.iter() {
            if !user.has_access(env_name.clone()) {
                continue;
            }
            for pk in self.public_keys.iter() {
                if pk.type_ == PublicKeyType::Machine && pk.id == user.id {
                    public_keys.push(pk.clone());
                    continue;
                }

                if let Some(passkey) = &pk.passkey
                    && passkey.user_id == user.id
                {
                    public_keys.push(pk.clone());
                }
            }
        }

        public_keys
    }

    fn compile_lock_file(&self, content: String) -> anyhow::Result<HashMap<String, String>> {
        let mut lock_values: HashMap<String, String> = HashMap::new();

        for public_key in &self.public_keys {
            let encrypted_key = public_key.encrypt(content.clone().into_bytes())?;
            lock_values.insert(public_key.id.clone(), encrypted_key);
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
