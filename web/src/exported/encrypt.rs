use crate::auth::login_user_bounded;
use secretpass_core::{LoginParams, StoredPublicKey};
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

    let _ = login_user_bounded(&params.login_params)
        .await
        .map_err(|err| JsError::new(&format!("Error authorizing user: {}", err)))?;

    let mut output: HashMap<String, String> = HashMap::new();
    for pk in params.public_keys {
        let key_id = pk.id.clone();
        let encrypted_value = pk
            .encrypt(params.secret_value.clone().into_bytes())
            .map_err(|err| JsError::new(&format!("Error encrypting secret: {}", err)))?;
        output.insert(key_id, encrypted_value);
    }

    serde_json::to_string(&output)
        .map_err(|err| JsError::new(&format!("Error serializing output: {}", err)))
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

    let (_, session) = login_user_bounded(&params.login_params)
        .await
        .map_err(|err| JsError::new(&format!("Error authorizing user: {}", err)))?;

    let plain_value = session
        .decrypt(&params.encrypted_secret)
        .map_err(|err| JsError::new(&format!("Error decrypting secret: {}", err)))?;

    String::from_utf8(plain_value)
        .map_err(|err| JsError::new(&format!("Error decoding decrypted secret: {}", err)))
}

#[derive(Deserialize, Tsify)]
pub struct ReEncryptPackageParams {
    login_params: LoginParams,
    target_key: StoredPublicKey,
    secrets_package: HashMap<String, HashMap<String, String>>,
}

#[wasm_bindgen]
pub async fn reencrypt_package(params: String) -> Result<String, JsError> {
    let params: ReEncryptPackageParams = serde_json::from_str(&params)
        .map_err(|err| JsError::new(&format!("Error deserializing params: {}", err)))?;

    let (_, session) = login_user_bounded(&params.login_params)
        .await
        .map_err(|err| JsError::new(&format!("Error authorizing user: {}", err)))?;

    let mut output: HashMap<String, HashMap<String, String>> = Default::default();
    for (env, encrypted_package) in &params.secrets_package {
        let decrypted_package = session.decrypt_package(encrypted_package).map_err(|err| {
            JsError::new(&format!("Error decrypting secrets from {}: {}", env, err))
        })?;

        let encrypted_package = params
            .target_key
            .encrypt_package(decrypted_package)
            .map_err(|err| {
                JsError::new(&format!("Error encrypting secrets for {}: {}", env, err))
            })?;
        output.insert(env.clone(), encrypted_package);
    }

    serde_json::to_string(&output)
        .map_err(|err| JsError::new(&format!("Error serializing output: {}", err)))
}

#[derive(Deserialize, Tsify)]
pub struct DecryptEnvPackageParams {
    login_params: LoginParams,
    secrets_package: HashMap<String, String>,
}

#[wasm_bindgen]
pub async fn decrypt_env_package(params: String) -> Result<String, JsError> {
    let params: DecryptEnvPackageParams = serde_json::from_str(&params)
        .map_err(|err| JsError::new(&format!("Error deserializing params: {}", err)))?;

    let (_, session) = login_user_bounded(&params.login_params)
        .await
        .map_err(|err| JsError::new(&format!("Error authorizing user: {}", err)))?;

    let mut output: HashMap<String, String> = Default::default();
    let decrypted_package = session
        .decrypt_package(&params.secrets_package)
        .map_err(|err| JsError::new(&format!("Error decrypting secrets: {}", err)))?;
    for (env, decrypted_secret) in decrypted_package {
        output.insert(
            env.clone(),
            String::from_utf8(decrypted_secret).map_err(|err| {
                JsError::new(&format!(
                    "Error converting decrypted secret to string: {}",
                    err
                ))
            })?,
        );
    }

    serde_json::to_string(&output)
        .map_err(|err| JsError::new(&format!("Error serializing output: {}", err)))
}
