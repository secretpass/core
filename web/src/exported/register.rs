use crate::auth::{RegistrationParams, register_user_bounded};
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn register_user(params: String) -> Result<String, JsError> {
    let params: RegistrationParams =
        serde_json::from_str(&params).map_err(|err| JsError::new(&err.to_string()))?;

    let public_key = register_user_bounded(params)
        .await
        .map_err(|err| JsError::new(&err.to_string()))?;

    let public_key_json =
        serde_json::to_string(&public_key).map_err(|err| JsError::new(&err.to_string()))?;

    Ok(public_key_json)
}
