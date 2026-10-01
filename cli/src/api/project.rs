use crate::api::dto::CreateProjectDTO;
use crate::project::{BackendManager, setup_working_directory};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use secretpass_core::{OperationMode, SecretManagerConfig};
use std::sync::{Mutex, OnceLock};

pub static ACTIVE_MANAGER: OnceLock<Mutex<BackendManager>> = OnceLock::new();

pub fn init_config(working_dir: Option<String>, is_cloud: bool) -> anyhow::Result<()> {
    setup_working_directory(working_dir, is_cloud);
    if is_cloud {
        return Ok(());
    }

    if let Some(manager) = BackendManager::load()? {
        ACTIVE_MANAGER.get_or_init(|| Mutex::new(manager));
    }
    Ok(())
}

async fn get_config() -> Result<Json<BackendManager>, (StatusCode, &'static str)> {
    match ACTIVE_MANAGER.get() {
        Some(config) => Ok(Json(config.lock().unwrap().clone())),
        None => Err((StatusCode::NOT_FOUND, "Secretpass project not found")),
    }
}

async fn create_config(
    Json(data): Json<CreateProjectDTO>,
) -> Result<Json<BackendManager>, (StatusCode, String)> {
    let current_config = ACTIVE_MANAGER.get();
    if current_config.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("Secretpass project already exists"),
        ));
    }

    let config = BackendManager {
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

    ACTIVE_MANAGER.set(Mutex::new(config)).unwrap();

    let mut config = ACTIVE_MANAGER.get().unwrap().lock().unwrap();
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
