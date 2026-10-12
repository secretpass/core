use crate::api::dto::{CreateOrUpdateUserDTO, UserLockPackageRequestDTO};
use crate::manager::{get_readonly_manager, get_readwrite_manager};
use crate::project::{read_secrets_value, write_secret_value};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use secretpass_core::LoginParams;

async fn get_user_public_keys(
    Query(params): Query<UserLockPackageRequestDTO>,
) -> Result<Json<LoginParams>, (StatusCode, String)> {
    let manager = get_readonly_manager()?;

    let user = manager
        .get_user_by_username(&params.username)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let public_keys = manager.get_user_public_keys(&user).ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("No public keys found for {}", params.username),
        )
    })?;

    Ok(Json(LoginParams {
        user: user.clone(),
        project: manager.config.project,
        public_keys,
    }))
}

async fn create_or_update_user(
    Json(data): Json<CreateOrUpdateUserDTO>,
) -> Result<(), (StatusCode, String)> {
    let mut manager = get_readwrite_manager().map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error loading backend secret manager: {:?}", err),
        )
    })?;

    // Check if the user exists
    if let Some(user) = &data.user
        && manager.get_user_by_id(&user.id).is_none()
    {
        manager.config.users.push(user.clone());
    }

    let key_id = &data.public_key.id;
    if manager.get_public_key(key_id).is_none() {
        manager.public_keys.push(data.public_key.clone());
    }

    for (env, env_secrets) in &data.secret_values {
        for (secret_name, secret_value) in env_secrets {
            let mut current = read_secrets_value(env, secret_name)
                .map_err(|err| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Error reading secret value: {:?}", err),
                    )
                })?
                .ok_or_else(|| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Secret {} not found", secret_name),
                    )
                })?;
            current.insert(key_id.clone(), secret_value.clone());

            write_secret_value(env, secret_name, current).map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error writing secret value: {:?}", err),
                )
            })?;
        }
    }

    manager.write().map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error writing config data: {:?}", err),
        )
    })?;

    Ok(())
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/api/user/login-params", get(get_user_public_keys))
        .route("/api/user", post(create_or_update_user))
}
