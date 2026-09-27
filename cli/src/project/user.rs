use crate::project::environment::EnvironmentAccess;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserLevel {
    Admin,
    Standard,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDefinition {
    pub username: String,
    pub name: String,
    pub level: UserLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<EnvironmentAccess>>,
}
