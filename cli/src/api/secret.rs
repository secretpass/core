use crate::api::dto::CreateOrUpdateSecretDTO;
use crate::manager::get_readwrite_manager;
use crate::project::{delete_secret, write_secret_value};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::put;
use axum::{Json, Router};

async fn create_or_update_secret(
    Path((env_name, secret_name)): Path<(String, String)>,
    Json(data): Json<CreateOrUpdateSecretDTO>,
) -> Result<(), (StatusCode, String)> {
    let mut manager = get_readwrite_manager()?;

    // Confirm the environment exists
    let _ = manager.get_environment(&env_name).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Environment {env_name} not found"),
        )
    })?;

    // Write the secret value
    write_secret_value(&env_name, &secret_name, data.value.clone()).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error writing secret value: {}", err),
        )
    })?;

    // Update the config
    manager
        .add_or_update_secret_definition(
            &env_name,
            &secret_name,
            data.data_type,
            data.description.clone(),
        )
        .ok_or_else(|| {
            (
                StatusCode::CONFLICT,
                format!("Environment {env_name} not found"),
            )
        })?;

    manager.write().map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error writing config: {}", err),
        )
    })?;

    Ok(())
}

async fn delete_secret_route(
    Path((env_name, secret_name)): Path<(String, String)>,
) -> Result<(), (StatusCode, String)> {
    let mut manager = get_readwrite_manager()?;

    delete_secret(&env_name, &secret_name).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error deleting secret: {}", err),
        )
    })?;

    manager.delete_secret_definition(&env_name, &secret_name);

    manager.write().map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error writing config: {}", err),
        )
    })?;

    Ok(())
}

pub fn secrets_routes() -> Router {
    Router::new().route(
        "/api/secrets/{env_name}/{secret_name}",
        put(create_or_update_secret).delete(delete_secret_route),
    )
}
