use crate::auth::{LoginParams, login_user_bounded};
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn authorize_operation(params: String) -> Result<(), JsError> {
    let params: LoginParams = serde_json::from_str(&params)?;

    let _ = login_user_bounded(params)
        .await
        .map_err(|err| JsError::new(&format!("Error authorizing in user: {}", err)))?;

    Ok(())
}
