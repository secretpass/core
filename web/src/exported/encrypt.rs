use crate::auth::{LoginParams, login_user_bounded};
use secretpass_core::StoredPublicKey;
use secretpass_core::utils::{bin_decode, bin_encode};
use serde::Deserialize;
use std::collections::HashMap;
use tsify::Tsify;
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Deserialize, Tsify)]
pub struct EncryptSecretParams {
    login_params: LoginParams,
    public_keys: Vec<StoredPublicKey>,
    secret_value: String,
}

#[wasm_bindgen]
pub async fn encrypt_secret(params: String) -> Result<String, JsError> {
    let params: EncryptSecretParams = serde_json::from_str(params.as_str())?;

    let _ = login_user_bounded(params.login_params)
        .await
        .map_err(|err| JsError::new(&format!("Error logging in user: {}", err)))?;

    let mut output: HashMap<String, String> = HashMap::new();
    for pk in params.public_keys {
        let key_id = pk.id.clone();
        let encrypted_value = pk
            .encrypt(params.secret_value.clone().into_bytes())
            .map_err(|err| JsError::new(&format!("Error encrypting secret: {}", err)))?;
        output.insert(key_id, bin_encode(&encrypted_value));
    }

    let output = serde_json::to_string(&output)
        .map_err(|err| JsError::new(&format!("Error serializing output: {}", err)))?;

    Ok(output)
}

#[derive(Deserialize, Tsify)]
pub struct DecryptSecretParams {
    login_params: LoginParams,
    encrypted_secret: String,
}

#[wasm_bindgen]
pub async fn decrypt_secret(params: String) -> Result<String, JsError> {
    let params: DecryptSecretParams = serde_json::from_str(&params)
        .map_err(|err| JsError::new(&format!("Error deserializing params: {}", err)))?;

    let (_, session) = login_user_bounded(params.login_params)
        .await
        .map_err(|err| JsError::new(&format!("Error authorizing in user: {}", err)))?;

    let encrypted_value = bin_decode(&params.encrypted_secret)
        .map_err(|err| JsError::new(&format!("Error decoding encrypted secret: {}", err)))?;
    let plain_value = session
        .decrypt(encrypted_value)
        .map_err(|err| JsError::new(&format!("Error decrypting secret: {}", err)))?;

    let plain_str = String::from_utf8(plain_value)
        .map_err(|err| JsError::new(&format!("Error decoding decrypted secret: {}", err)))?;
    Ok(plain_str)
}
