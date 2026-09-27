use crate::project::{SecretManagerConfig, setup_working_directory};
use axum::routing::get;
use axum::{Json, Router};
use std::sync::{Mutex, OnceLock};

pub static ACTIVE_CONFIG: OnceLock<Mutex<SecretManagerConfig>> = OnceLock::new();

pub fn init_config(working_dir: Option<String>, is_cloud: bool) {
    setup_working_directory(working_dir);
    ACTIVE_CONFIG.get_or_init(|| Mutex::new(SecretManagerConfig::load(is_cloud)));
}

async fn get_project() -> Json<SecretManagerConfig> {
    let config = ACTIVE_CONFIG.get().unwrap().lock().unwrap().clone();
    Json(config)
}

pub fn project_routes() -> Router {
    Router::new().route("/api/project", get(get_project))
}
