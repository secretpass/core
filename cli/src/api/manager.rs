use crate::project::{BackendManager, setup_working_directory};
use axum::http::StatusCode;
use std::sync::{Mutex, MutexGuard, OnceLock};

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

pub fn get_readwrite_manager() -> Result<MutexGuard<'static, BackendManager>, (StatusCode, String)>
{
    let manager = ACTIVE_MANAGER
        .get()
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                "Secretpass project not loaded".to_string(),
            )
        })?
        .lock()
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get a secret manager lock: {}", err),
            )
        })?;

    Ok(manager)
}

pub fn get_readonly_manager() -> Result<BackendManager, (StatusCode, String)> {
    let manager = get_readwrite_manager()?;

    Ok(manager.clone())
}
