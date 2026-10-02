use crate::api::dto::CreateProjectDTO;
use crate::api::manager::{ACTIVE_MANAGER, get_readonly_manager};
use crate::project::BackendManager;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use secretpass_core::{OperationMode, SecretManagerConfig, SecretpassProject};
use std::sync::Mutex;

async fn get_project() -> Result<Json<SecretpassProject>, (StatusCode, String)> {
    let manager = get_readonly_manager()?;

    Ok(Json(manager.config.project))
}

async fn create_project(
    Json(data): Json<CreateProjectDTO>,
) -> Result<Json<SecretManagerConfig>, (StatusCode, String)> {
    let current_config = ACTIVE_MANAGER.get();
    if current_config.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("Secretpass project already exists"),
        ));
    }

    let mut manager = BackendManager {
        config: SecretManagerConfig {
            mode: OperationMode::Local,
            project: data.project,
            environments: data.environments,
            users: vec![data.user],
        },
        config_lock: Default::default(),
        public_keys: vec![data.public_key],
        public_keys_lock: Default::default(),
    };

    if let Err(err) = manager.write() {
        println!("Error writing secretpass config: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error writing config to disk: {}", err),
        ));
    }

    let config = manager.config.clone();
    ACTIVE_MANAGER.set(Mutex::new(manager)).unwrap();

    Ok(Json(config))
}

pub fn project_routes() -> Router {
    Router::new().route("/api/project", get(get_project).post(create_project))
}
