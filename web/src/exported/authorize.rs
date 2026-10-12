use crate::auth::login_user_bounded;
use secretpass_core::LoginParams;
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn authorize_operation(params: String) -> Result<String, JsError> {
    let params: LoginParams = serde_json::from_str(&params)?;

    let (pk, _) = login_user_bounded(&params)
        .await
        .map_err(|err| JsError::new(&format!("Error authorizing user: {}", err)))?;

    Ok(pk.id)
}
