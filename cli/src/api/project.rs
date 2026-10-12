use crate::api::dto::{CreateProjectDTO, UpdateProjectDTO};
use crate::manager::{ACTIVE_MANAGER, get_readonly_manager, get_readwrite_manager};
use crate::project::{BackendManager, get_working_directory};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use secretpass_core::{
    OperationMode, SecretManagerConfig, SecretpassProject, UserApplicationState,
};
use std::sync::Mutex;

async fn get_project() -> Result<Json<SecretpassProject>, (StatusCode, String)> {
    let manager = get_readonly_manager()?;

    Ok(Json(manager.config.project))
}

async fn get_project_state(
    Path(key_id): Path<String>,
) -> Result<Json<UserApplicationState>, (StatusCode, String)> {
    let manager = get_readonly_manager()?;
    let state = manager.get_user_application_state(&key_id)?;

    Ok(Json(state))
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
    let dir = get_working_directory();

    let mut manager = BackendManager {
        dir,
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

async fn update_project(Json(data): Json<UpdateProjectDTO>) -> Result<(), (StatusCode, String)> {
    let mut manager = get_readwrite_manager()?;

    if let Some(config) = data.config {
        manager.config = config;
    }

    if let Some(public_keys) = data.public_keys {
        manager.public_keys = public_keys;
    }

    if let Err(err) = manager.write() {
        println!("Error writing secretpass config: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error writing config to disk: {}", err),
        ));
    }

    Ok(())
}

pub fn project_routes() -> Router {
    Router::new()
        .route(
            "/api/project",
            get(get_project).post(create_project).patch(update_project),
        )
        .route("/api/project/state/{key_id}", get(get_project_state))
}
