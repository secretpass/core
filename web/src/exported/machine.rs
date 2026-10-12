use crate::auth::login_user_bounded;
use secretpass_core::{LoginParams, PrivateKey, PrivateKeySeed, PublicKeyType, StoredPublicKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tsify::Tsify;
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Deserialize, Tsify)]
pub struct GenerateMachineKeyParams {
    login_params: LoginParams,
    secrets_package: HashMap<String, HashMap<String, String>>,
}

#[derive(Serialize, Tsify)]
pub struct GenerateMachineKeyResult {
    private_key: String,
    public_key: StoredPublicKey,
    secrets_package: HashMap<String, HashMap<String, String>>,
}

#[wasm_bindgen]
pub async fn generate_machine_key(name: String, params: String) -> Result<String, JsError> {
    let params: GenerateMachineKeyParams = serde_json::from_str(&params)
        .map_err(|err| JsError::new(&format!("Error deserializing params: {}", err)))?;

    let (_, session) = login_user_bounded(&params.login_params)
        .await
        .map_err(|err| JsError::new(&format!("Error authorizing user: {}", err)))?;

    let (machine_key, seed) =
        PrivateKeySeed::generate_machine_key(params.login_params.project.algorithm)
            .map_err(|err| JsError::new(&format!("Error generating machine key: {}", err)))?;
    let private_key = PrivateKey::from_seed(seed)
        .map_err(|err| JsError::new(&format!("Error getting private key from seed: {}", err)))?;

    let public_key = private_key.public_key();

    let mut output: HashMap<String, HashMap<String, String>> = Default::default();
    for (env, encrypted_package) in &params.secrets_package {
        let decrypted_package = session.decrypt_package(encrypted_package).map_err(|err| {
            JsError::new(&format!("Error decrypting secrets from {}: {}", env, err))
        })?;

        let encrypted_package = public_key
            .encrypt_package(decrypted_package)
            .map_err(|err| {
                JsError::new(&format!("Error encrypting secrets for {}: {}", env, err))
            })?;
        output.insert(env.clone(), encrypted_package);
    }

    let public_key = StoredPublicKey {
        id: public_key.calculate_id(PublicKeyType::Machine),
        ecc: public_key.ecc(),
        kem: public_key.kem(),
        passkey: None,
        name,
        type_: PublicKeyType::Machine,
    };

    let result = GenerateMachineKeyResult {
        private_key: machine_key,
        public_key,
        secrets_package: output,
    };

    serde_json::to_string(&result)
        .map_err(|err| JsError::new(&format!("Error serializing result: {}", err)))
}
