use crate::api::dto::UserLockPackageRequestDTO;
use crate::api::manager::get_readonly_manager;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use secretpass_core::UserLockPackage;

async fn get_user_public_keys(
    Query(params): Query<UserLockPackageRequestDTO>,
) -> Result<Json<UserLockPackage>, (StatusCode, String)> {
    let manager = get_readonly_manager()?;

    let user = manager
        .get_user_by_username(&params.username)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let (public_keys, config_lock, public_keys_lock) = manager
        .get_user_lock_package(&user)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(UserLockPackage {
        user: user.clone(),
        project: manager.config.project,
        public_keys,
        config_lock,
        public_keys_lock,
    }))
}

pub fn user_routes() -> Router {
    Router::new().route("/api/user/lock-package", get(get_user_public_keys))
}
