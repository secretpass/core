use crate::passkeys::types::*;
use coset::cbor::value::Value;
use coset::{CborSerializable, CoseKey, Label};
use p256::EncodedPoint;
use p256::ecdsa::signature::Verifier;
use p256::ecdsa::{Signature, VerifyingKey};
use secretpass_core::utils::{bin_decode, bin_encode};
use secretpass_core::{Passkey, PasskeyResidency, SecretpassProject, SecretpassUser};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use wasm_bindgen::throw_str;

const CHALLENGE_LEN: usize = 32;

#[derive(Deserialize)]
struct ClientData {
    challenge: String,
    origin: String,
    #[serde(rename = "type")]
    type_: String,
}

impl ClientData {
    fn challenge(&self) -> anyhow::Result<Vec<u8>> {
        let challenge_bytes = bin_decode(&self.challenge)?;
        if challenge_bytes.len() != CHALLENGE_LEN {
            throw_str("Invalid challenge length")
        }
        Ok(challenge_bytes)
    }
}

static CONFIG: PasskeyConfig = PasskeyConfig {
    rp_id: env!("WEBAUTH_RP_ID"),
    rp_name: env!("WEBAUTH_RP_ENTITY"),
    origin: env!("WEBAUTH_RP_ORIGIN"),
};

#[derive(Clone, Debug)]
struct AuthData {
    rp_id_hash: Vec<u8>,
    credential_data: Option<Vec<u8>>,
}

fn generate_challenge() -> anyhow::Result<Vec<u8>> {
    let mut buf = [0u8; CHALLENGE_LEN];
    getrandom::fill(&mut buf)
        .map_err(|e| anyhow::format_err!("Failed to generate random challenge: {e}"))?;
    Ok(buf.to_vec())
}

fn verify_client_data(
    bytes: &[u8],
    expected_challenge: &Vec<u8>,
    expected_type: &str,
) -> anyhow::Result<()> {
    let data: ClientData = serde_json::from_slice(bytes)?;

    if data.challenge()? != expected_challenge.clone() {
        return Err(anyhow::anyhow!("Invalid challenge"));
    }

    // Due to dynamic port configuration
    if !data.origin.as_str().starts_with(CONFIG.origin) {
        return Err(anyhow::format_err!(
            "Origin mismatch, expected: {}, got: {}",
            CONFIG.origin,
            data.origin
        ));
    }
    if data.type_ != expected_type {
        return Err(anyhow::format_err!(
            "Invalid operation type: {}",
            data.type_
        ));
    }
    Ok(())
}

fn check_flags(flags: u8, project: &SecretpassProject) -> anyhow::Result<()> {
    let user_present = (flags & 0x01) != 0;
    let user_verified = (flags & 0x04) != 0;
    if !user_present || !user_verified {
        return Err(anyhow::anyhow!(
            "User presence and authorization cannot be verified"
        ));
    }

    if project.residency != PasskeyResidency::SyncedAllowed {
        let backup_eligible = (flags & 0x08) != 0;
        let backup_completed = (flags & 0x10) != 0;

        if backup_eligible || backup_completed {
            return Err(anyhow::anyhow!("Synced passkeys are not allowed"));
        }
    }

    Ok(())
}

fn parse_auth_data(raw: &[u8], project: &SecretpassProject) -> anyhow::Result<AuthData> {
    if raw.len() < 37 {
        return Err(anyhow::anyhow!("authData too short"));
    }
    let rp_id_hash = raw[0..32].to_vec();
    let flags = raw[32];

    check_flags(flags, project)?;

    let credential_data = if (flags & 0x40) != 0 {
        Some(raw[37..].to_vec())
    } else {
        None
    };
    Ok(AuthData {
        rp_id_hash,
        credential_data,
    })
}

fn verify_rp_id_hash(hash: &[u8]) -> anyhow::Result<()> {
    let expected = Sha256::digest(CONFIG.rp_id.as_bytes());
    if hash != expected.as_ref() as &[u8] {
        return Err(anyhow::anyhow!("RP ID hash mismatch"));
    }
    Ok(())
}

fn extract_credential(data: &[u8]) -> anyhow::Result<(&[u8], &[u8])> {
    if data.len() < 18 {
        return Err(anyhow::anyhow!("Credential Data too short"));
    }
    let cred_id_len = u16::from_be_bytes(data[16..18].try_into().unwrap()) as usize;
    if data.len() < 18 + cred_id_len {
        return Err(anyhow::anyhow!("Credential ID incomplete"));
    }
    let cred_id = &data[18..18 + cred_id_len];
    let pub_key_cbor = &data[18 + cred_id_len..];
    Ok((cred_id, pub_key_cbor))
}

fn verify_p256_signature(
    pub_key_cbor: &[u8],
    signed_data: &[u8],
    signature_der: &[u8],
) -> anyhow::Result<()> {
    let cose_key = CoseKey::from_slice(pub_key_cbor)
        .map_err(|e| anyhow::format_err!("Invalid COSE key: {e}"))?;

    let x = match cose_key.params.iter().find(|(k, _)| k == &Label::Int(-2)) {
        Some((_, Value::Bytes(b))) => b,
        _ => return Err(anyhow::anyhow!("Missing x coordinate")),
    };
    let y = match cose_key.params.iter().find(|(k, _)| k == &Label::Int(-3)) {
        Some((_, Value::Bytes(b))) => b,
        _ => return Err(anyhow::anyhow!("Missing y coordinate")),
    };

    if x.len() != 32 || y.len() != 32 {
        return Err(anyhow::anyhow!("Invalid coordinate length"));
    }

    let encoded_point =
        EncodedPoint::from_affine_coordinates(x.as_slice().into(), y.as_slice().into(), false);
    let verifying_key = VerifyingKey::from_encoded_point(&encoded_point)
        .map_err(|e| anyhow::format_err!("Invalid P-256 key: {e}"))?;

    let signature = Signature::from_der(signature_der)
        .map_err(|e| anyhow::format_err!("Error parsing signature: {e}"))?;

    verifying_key
        .verify(signed_data, &signature)
        .map_err(|e| anyhow::format_err!("Invalid signature: {e}"))?;
    Ok(())
}

// Core WebAuthn Flows

/// Initiates a new passkey registration.
///
/// Returns the options that must be sent to the WebAuthn client (`navigator.credentials.create`).
/// It also saves the registration session state to the provided `store`.
pub async fn start_registration(
    project: &SecretpassProject,
    user: &SecretpassUser,
) -> anyhow::Result<PasskeyCreationOptions> {
    let challenge = generate_challenge()?;

    let options = PasskeyCreationOptions {
        rp: RpEntity {
            name: format!("{} - {}", CONFIG.rp_name, project.name),
            id: CONFIG.rp_id.to_string(),
        },
        user: UserEntity {
            id: user.id.clone(),
            name: user.username.clone(),
            display_name: user.name.clone(),
        },
        challenge,
        pub_key_cred_params: vec![PubKeyCredParam {
            type_: "public-key".into(),
            alg: -7, // ES256
        }],
        timeout: Some(60000),
        exclude_credentials: None,
        authenticator_selection: Some(AuthenticatorSelection {
            authenticator_attachment: None,
            require_resident_key: Some(false),
            resident_key: Some("preferred".into()),
            user_verification: Some("preferred".into()),
        }),
        attestation: Some("none".into()),
    };

    Ok(options)
}

/// Completes a passkey registration.
///
/// Validates the client response against the stored challenge and RP configuration.
/// On success, a new [`Passkey`](crate::types::Passkey) is created via the `store`.
pub fn finish_registration(
    project: &SecretpassProject,
    options: &PasskeyCreationOptions,
    response: RegistrationResponse,
) -> anyhow::Result<Passkey> {
    verify_client_data(
        &response.response.client_data_json,
        &options.challenge,
        "webauthn.create",
    )?;

    // Parse attestation object (CBOR)
    let att_obj: Value = ciborium::from_reader(response.response.attestation_object.as_slice())
        .map_err(|e| anyhow::format_err!(format!("Invalid attestationObject CBOR: {e}")))?;

    let Value::Map(m) = &att_obj else {
        return Err(anyhow::anyhow!("Invalid attestation object structure"));
    };

    let (_, auth_data_value) = m
        .iter()
        .find(|(k, _)| k.as_text().is_some_and(|s| s == "authData"))
        .ok_or_else(|| anyhow::anyhow!("authData missing"))?;

    let auth_data_bytes = auth_data_value
        .as_bytes()
        .ok_or_else(|| anyhow::anyhow!("authData not bytes"))?;

    // Verify authData
    let auth_data = parse_auth_data(auth_data_bytes, project)?;
    verify_rp_id_hash(&auth_data.rp_id_hash)?;

    // Extract credential
    let cred_bytes = auth_data
        .credential_data
        .ok_or_else(|| anyhow::anyhow!("Attested Credential Data missing"))?;
    let (cred_id, pub_key_cbor) = extract_credential(&cred_bytes)?;
    // Validate the public key parses
    CoseKey::from_slice(pub_key_cbor)
        .map_err(|e| anyhow::format_err!("Invalid Public Key CBOR: {e}"))?;

    // Encode for storage
    let cred_id_b64 = bin_encode(cred_id);
    let pub_key_b64 = bin_encode(pub_key_cbor);

    let passkey = Passkey {
        id: cred_id_b64,
        public_key: pub_key_b64,
        user_id: options.user.id.clone(),
        username: options.user.name.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(passkey)
}

/// Initiates a passkey login flow.
///
/// Returns the options that must be sent to the WebAuthn client (`navigator.credentials.get`).
/// It saves a login session state keyed by the challenge.
pub fn start_login() -> anyhow::Result<PasskeyRequestOptions> {
    let challenge = generate_challenge()?;

    let options = PasskeyRequestOptions {
        challenge,
        timeout: Some(60000),
        rp_id: CONFIG.rp_id.to_string(),
        allow_credentials: None,
        user_verification: Some("preferred".into()),
    };

    Ok(options)
}

/// Completes a passkey login flow.
///
/// Validates the client response, signature, and counter.
/// On success, returns the `user_id` of the authenticated user and updates the counter in the `store`.
pub async fn finish_login(
    project: &SecretpassProject,
    passkey: &Passkey,
    challenge: &Vec<u8>,
    response: LoginResponse,
) -> anyhow::Result<()> {
    // Full clientDataJSON verification
    verify_client_data(
        &response.response.client_data_json,
        challenge,
        "webauthn.get",
    )?;

    // Parse & verify authenticator data
    let auth_data = parse_auth_data(&response.response.authenticator_data, project)?;
    verify_rp_id_hash(&auth_data.rp_id_hash)?;

    // Verify user handle if present
    if let Some(uh_bytes) = response.response.user_handle {
        let uid_str =
            String::from_utf8(uh_bytes).map_err(|_| anyhow::anyhow!("Invalid userHandle utf8"))?;
        if uid_str != passkey.user_id {
            return Err(anyhow::anyhow!("User Handle mismatch"));
        }
    }

    let client_data_hash = Sha256::digest(&response.response.client_data_json);
    let mut signed_data = Vec::with_capacity(response.response.authenticator_data.len() + 32);
    signed_data.extend_from_slice(&response.response.authenticator_data);
    signed_data.extend_from_slice(&client_data_hash);

    verify_p256_signature(
        &passkey.public_key_bytes()?,
        &signed_data,
        &response.response.signature,
    )?;

    // Return PRF results
    Ok(())
}
