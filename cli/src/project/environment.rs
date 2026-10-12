use crate::project::secrets::read_secrets_value;
use axum::http::StatusCode;
use secretpass_core::{SecretpassEnvironment, StoredPublicKey};
use std::collections::HashMap;

pub fn get_environment_encrypted_secrets(
    environment: &SecretpassEnvironment,
    public_key: &StoredPublicKey,
) -> Result<HashMap<String, String>, (StatusCode, String)> {
    let mut encrypted_secrets: HashMap<String, String> = HashMap::new();

    for secret in environment.secrets.iter() {
        let secret_value = read_secrets_value(&environment.name, &secret.name).map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error reading {}: {}", secret.name, err),
            )
        })?;
        if let Some(value) = secret_value {
            let key_value = value.get(&public_key.id).ok_or_else(|| {
                (
                    StatusCode::CONFLICT,
                    format!("Key {} not found in secret {}", public_key.id, secret.name),
                )
            })?;
            encrypted_secrets.insert(secret.name.clone(), key_value.clone());
        }
    }

    Ok(encrypted_secrets)
}
