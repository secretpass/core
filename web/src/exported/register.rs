use crate::auth::{RegistrationParams, register_user_bounded};
use secretpass_core::utils::{bin_deserialize, bin_serialize};
use secretpass_core::{SecretpassProject, SecretpassUser, StoredPublicKey};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn register_user(name: String, params: String) -> Result<String, JsError> {
    let params: RegistrationParams = serde_json::from_str(&params)
        .map_err(|err| JsError::new(&format!("Error parsing config: {:?}", err)))?;

    let public_key = register_user_bounded(name, params)
        .await
        .map_err(|err| JsError::new(&format!("Error registering user: {:?}", err)))?;

    let public_key_json =
        serde_json::to_string(&public_key).map_err(|err| JsError::new(&format!("{:?}", err)))?;

    Ok(public_key_json)
}

// Performed by admin
#[wasm_bindgen]
pub fn generate_registration_config(params: String) -> Result<String, JsError> {
    let params: RegistrationParams =
        serde_json::from_str(&params).map_err(|err| JsError::new(&format!("{:?}", err)))?;

    // Intentionally make the config non-human friendly but friendly for copy and paste
    bin_serialize(&params).map_err(|err| JsError::new(&err.to_string()))
}

// Performed by the user
#[wasm_bindgen]
pub fn decode_registration_config(params: String) -> Result<String, JsError> {
    let params: RegistrationParams = bin_deserialize(&params)
        .map_err(|err| JsError::new(&format!("Invalid registration config: {}", err)))?;

    let params_json =
        serde_json::to_string(&params).map_err(|err| JsError::new(&err.to_string()))?;
    Ok(params_json)
}

#[derive(Debug, Serialize, Deserialize, Tsify)]
pub struct KeyCreationResult {
    pub project: SecretpassProject,
    pub user: SecretpassUser,
    pub public_key: StoredPublicKey,
}

// Performed by the user
#[wasm_bindgen]
pub async fn register_key_from_config(name: String, params: String) -> Result<String, JsError> {
    let params: RegistrationParams = bin_deserialize(&params)
        .map_err(|err| JsError::new(&format!("Invalid registration config: {}", err)))?;

    let user = params.user.clone();
    let project = params.project.clone();

    let public_key = register_user_bounded(name, params)
        .await
        .map_err(|err| JsError::new(&err.to_string()))?;

    let result = KeyCreationResult {
        user,
        project,
        public_key,
    };

    // Intentionally make the config non-human friendly but friendly for copy and paste
    bin_serialize(&result).map_err(|err| JsError::new(&err.to_string()))
}

// Performed by the admin
#[wasm_bindgen]
pub async fn decode_registration_result(params: String) -> Result<String, JsError> {
    let result: KeyCreationResult = bin_deserialize(&params)
        .map_err(|err| JsError::new(&format!("Invalid passkey config: {}", err)))?;

    let json_result =
        serde_json::to_string(&result).map_err(|err| JsError::new(&err.to_string()))?;

    Ok(json_result)
}
