use crate::api::manager::get_readonly_manager;
use crate::project::BackendManager;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use secretpass_core::EnvironmentLockPackage;
use std::collections::HashMap;

async fn get_environment_lock_package(
    Path((env_name, key_id)): Path<(String, String)>,
) -> Result<Json<EnvironmentLockPackage>, (StatusCode, String)> {
    let manager = get_readonly_manager()?;

    let environment = manager.get_environment(&env_name).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Environment {env_name} not found"),
        )
    })?;

    let public_keys = manager.get_public_keys_for_env(env_name.clone());
    let mut encrypted_secrets: HashMap<String, Option<String>> = HashMap::new();

    for secret in environment.secrets.iter() {
        let secret_value =
            BackendManager::read_secrets_value(&env_name, &secret.name).map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error reading {}: {}", secret.name, err),
                )
            })?;
        if let Some(value) = secret_value {
            let key_value = value.get(&key_id).ok_or_else(|| {
                (
                    StatusCode::CONFLICT,
                    format!("Key {key_id} not found in secret {}", secret.name),
                )
            })?;
            encrypted_secrets.insert(secret.name.clone(), Some(key_value.clone()));
        } else {
            encrypted_secrets.insert(secret.name.clone(), None);
        }
    }

    Ok(Json(EnvironmentLockPackage {
        public_keys,
        encrypted_secrets,
    }))
}

pub fn environment_routes() -> Router {
    Router::new().route(
        "/api/environment/{env_name}/lock-package/{key_id}",
        get(get_environment_lock_package),
    )
}
