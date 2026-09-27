use crate::project::directory::get_working_directory;
use crate::project::environment::EnvironmentDefinition;
use crate::project::secret::{SecretDefinition, SecretValue};
use crate::project::user::UserDefinition;
use secretpass_core::{OperationMode, SecretpassProject, StoredPublicKey};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretManagerConfig {
    pub mode: OperationMode,
    pub project: SecretpassProject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<EnvironmentDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<SecretDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_new: Option<bool>,
}

impl Default for SecretManagerConfig {
    fn default() -> Self {
        Self {
            mode: OperationMode::Local,
            project: SecretpassProject::default(),
            environments: None,
            secrets: None,
            users: None,
            is_new: None,
        }
    }
}

impl SecretManagerConfig {
    pub fn load(is_cloud: bool) -> Self {
        let project_path = get_working_directory().path().join(".spass");
        let config_path = project_path.join("config.yaml");

        // Check if the project exists
        if !project_path.is_dir() || !config_path.is_file() {
            // This a new project or cloud setup
            return Self::new(is_cloud);
        }

        let config_content =
            fs::read(config_path).expect("Error reading config file .spass/config.yaml");

        yaml_serde::from_slice::<SecretManagerConfig>(config_content.as_slice())
            .expect("Error parsing config file .spass/config.yaml")
    }

    fn new(is_cloud: bool) -> Self {
        let mut config = Self::default();
        if is_cloud {
            config.mode = OperationMode::Cloud;
        }

        config.is_new = Some(true);
        config
    }

    fn add_user(
        &mut self,
        definition: UserDefinition,
        public_key: StoredPublicKey,
        secrets: Vec<SecretValue>,
    ) {
        if self.users.is_none() {
            self.users = Some(Vec::new());
        }

        self.users.as_mut().unwrap().push(definition);
    }
}
