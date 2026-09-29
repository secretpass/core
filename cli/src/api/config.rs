use crate::api::dto::CreateProjectDTO;
use crate::project::{SecretManagerConfig, setup_working_directory};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use secretpass_core::OperationMode;
use std::sync::{Mutex, OnceLock};

pub static ACTIVE_CONFIG: OnceLock<Mutex<SecretManagerConfig>> = OnceLock::new();

pub fn init_config(working_dir: Option<String>, is_cloud: bool) {
    setup_working_directory(working_dir, is_cloud);
    if is_cloud {
        return;
    }

    if let Some(config) = SecretManagerConfig::load() {
        ACTIVE_CONFIG.get_or_init(|| Mutex::new(config));
    }
}

async fn get_config() -> Result<Json<SecretManagerConfig>, (StatusCode, &'static str)> {
    match ACTIVE_CONFIG.get() {
        Some(config) => Ok(Json(config.lock().unwrap().clone())),
        None => Err((StatusCode::NOT_FOUND, "Secretpass project not found")),
    }
}

async fn create_config(
    Json(data): Json<CreateProjectDTO>,
) -> Result<Json<SecretManagerConfig>, (StatusCode, String)> {
    let current_config = ACTIVE_CONFIG.get();
    if current_config.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("Secretpass project already exists"),
        ));
    }

    let config = SecretManagerConfig {
        mode: OperationMode::Local,
        project: data.project,
        environments: data.environments,
        users: vec![data.user],
        secrets: vec![],
        public_keys: vec![data.public_key],
    };

    ACTIVE_CONFIG.set(Mutex::new(config)).unwrap();

    let config = ACTIVE_CONFIG.get().unwrap().lock().unwrap().clone();
    if let Err(err) = config.write() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error writing config to disk: {}", err),
        ));
    }

    Ok(Json(config))
}

pub fn config_routes() -> Router {
    Router::new().route("/api/config", get(get_config).post(create_config))
}
