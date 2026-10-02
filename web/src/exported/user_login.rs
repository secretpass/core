use crate::auth::{LoginParams, login_user_bounded};
use secretpass_core::utils::bin_decode;
use secretpass_core::{SecretManagerConfig, StoredPublicKey, UserLockPackage};
use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize, Tsify)]
pub struct ProcessUserLoginResponse {
    pub public_key: StoredPublicKey,
    pub config: SecretManagerConfig,
    pub public_keys: Vec<StoredPublicKey>,
}

#[wasm_bindgen]
pub async fn process_user_login(params: String) -> Result<String, JsError> {
    let lock_pkg: UserLockPackage = serde_json::from_str(&params)
        .map_err(|err| JsError::new(&format!("Error parsing lock package: {}", err)))?;

    let login_params = LoginParams {
        user: lock_pkg.user.clone(),
        project: lock_pkg.project.clone(),
        public_keys: lock_pkg.public_keys.clone(),
    };

    let (public_key, secure_session) = login_user_bounded(login_params)
        .await
        .map_err(|err| JsError::new(&format!("Error logging in user: {}", err)))?;

    let config_cipher = lock_pkg
        .config_lock
        .get(&public_key.id)
        .ok_or(JsError::new("Config lock for passkey key not found"))?;
    let public_key_cipher = lock_pkg
        .public_keys_lock
        .get(&public_key.id)
        .ok_or(JsError::new("Public keys lock for passkey not found"))?;

    let config_content = secure_session
        .decrypt(bin_decode(config_cipher).map_err(|err| {
            JsError::new(&format!("Error parsing base64 config content: {}", err))
        })?)
        .map_err(|err| JsError::new(&format!("Error decrypting config content: {}", err)))?;

    let public_keys_content = secure_session
        .decrypt(bin_decode(public_key_cipher).map_err(|err| {
            JsError::new(&format!(
                "Error parsing base64 public keys content: {}",
                err
            ))
        })?)
        .map_err(|err| JsError::new(&format!("Error decrypting public keys content: {}", err)))?;

    let config: SecretManagerConfig = serde_json::from_slice(&config_content)
        .map_err(|err| JsError::new(&format!("Error parsing config content: {}", err)))?;

    let public_keys: Vec<StoredPublicKey> = serde_json::from_slice(&public_keys_content)
        .map_err(|err| JsError::new(&format!("Error parsing public keys content: {}", err)))?;

    let response = ProcessUserLoginResponse {
        public_key,
        config,
        public_keys,
    };

    let content = serde_json::to_string(&response)
        .map_err(|err| JsError::new(&format!("Error serializing response: {}", err)))?;

    Ok(content)
}
