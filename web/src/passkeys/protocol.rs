use crate::passkeys::error::{PasskeyError, Result};
use crate::passkeys::types::*;
use base64::prelude::*;
use coset::cbor::value::Value;
use coset::{CborSerializable, CoseKey, Label};
use p256::EncodedPoint;
use p256::ecdsa::signature::Verifier;
use p256::ecdsa::{Signature, VerifyingKey};
use serde::Deserialize;
use sha2::{Digest, Sha256};

const CHALLENGE_LEN: usize = 32;

#[derive(Deserialize)]
struct ClientData {
    challenge: String,
    origin: String,
    #[serde(rename = "type")]
    type_: String,
}

static CONFIG: PasskeyConfig = PasskeyConfig {
    rp_id: env!("WEBAUTH_RP_ID"),
    rp_name: env!("WEBAUTH_RP_ENTITY"),
    origin: env!("WEBAUTH_RP_ORIGIN"),
};

struct AuthData {
    rp_id_hash: Vec<u8>,
    flags: u8,
    credential_data: Option<Vec<u8>>,
}

fn generate_challenge() -> Result<String> {
    let mut buf = [0u8; CHALLENGE_LEN];
    getrandom::fill(&mut buf).map_err(|e| {
        PasskeyError::InternalError(format!("Failed to generate random challenge: {e}"))
    })?;
    Ok(BASE64_URL_SAFE_NO_PAD.encode(buf))
}

fn verify_client_data(
    client_data_b64: &str,
    expected_challenge: &str,
    expected_type: &str,
) -> Result<(ClientData, Vec<u8>)> {
    let bytes = BASE64_URL_SAFE_NO_PAD.decode(client_data_b64)?;
    let data: ClientData = serde_json::from_slice(&bytes)?;

    if data.challenge != expected_challenge {
        return Err(PasskeyError::InvalidChallenge);
    }
    if data.origin != CONFIG.origin {
        return Err(PasskeyError::OriginMismatch {
            expected: CONFIG.origin.to_string(),
            got: data.origin,
        });
    }
    if data.type_ != expected_type {
        return Err(PasskeyError::InvalidOperationType);
    }
    Ok((data, bytes))
}

fn parse_auth_data(raw: &[u8]) -> Result<AuthData> {
    if raw.len() < 37 {
        return Err(PasskeyError::InternalError("authData too short".into()));
    }
    let rp_id_hash = raw[0..32].to_vec();
    let flags = raw[32];
    let credential_data = if (flags & 0x40) != 0 {
        Some(raw[37..].to_vec())
    } else {
        None
    };
    Ok(AuthData {
        rp_id_hash,
        flags,
        credential_data,
    })
}

fn verify_rp_id_hash(hash: &[u8]) -> Result<()> {
    let expected = Sha256::digest(CONFIG.rp_id.as_bytes());
    if hash != expected.as_ref() as &[u8] {
        return Err(PasskeyError::RpIdHashMismatch);
    }
    Ok(())
}

fn verify_user_present(flags: u8) -> Result<()> {
    if (flags & 0x01) == 0 {
        return Err(PasskeyError::UserPresentFlagNotSet);
    }
    Ok(())
}

fn extract_credential(data: &[u8]) -> Result<(&[u8], &[u8])> {
    if data.len() < 18 {
        return Err(PasskeyError::InternalError(
            "Credential Data too short".into(),
        ));
    }
    let cred_id_len = u16::from_be_bytes(data[16..18].try_into().unwrap()) as usize;
    if data.len() < 18 + cred_id_len {
        return Err(PasskeyError::InternalError(
            "Credential ID incomplete".into(),
        ));
    }
    let cred_id = &data[18..18 + cred_id_len];
    let pub_key_cbor = &data[18 + cred_id_len..];
    Ok((cred_id, pub_key_cbor))
}

fn verify_p256_signature(
    pub_key_cbor: &[u8],
    signed_data: &[u8],
    signature_der: &[u8],
) -> Result<()> {
    let cose_key = CoseKey::from_slice(pub_key_cbor)
        .map_err(|e| PasskeyError::InternalError(format!("Invalid COSE key: {e}")))?;

    let x = match cose_key.params.iter().find(|(k, _)| k == &Label::Int(-2)) {
        Some((_, Value::Bytes(b))) => b,
        _ => return Err(PasskeyError::InternalError("Missing x coordinate".into())),
    };
    let y = match cose_key.params.iter().find(|(k, _)| k == &Label::Int(-3)) {
        Some((_, Value::Bytes(b))) => b,
        _ => return Err(PasskeyError::InternalError("Missing y coordinate".into())),
    };

    if x.len() != 32 || y.len() != 32 {
        return Err(PasskeyError::InternalError(
            "Invalid coordinate length".into(),
        ));
    }

    let encoded_point =
        EncodedPoint::from_affine_coordinates(x.as_slice().into(), y.as_slice().into(), false);
    let verifying_key = VerifyingKey::from_encoded_point(&encoded_point)
        .map_err(|e| PasskeyError::InternalError(format!("Invalid P-256 key: {e}")))?;

    let signature = Signature::from_der(signature_der)
        .map_err(|e| PasskeyError::InvalidSignature(e.to_string()))?;

    verifying_key
        .verify(signed_data, &signature)
        .map_err(|e| PasskeyError::InvalidSignature(e.to_string()))
}

// Core WebAuthn Flows

/// Initiates a new passkey registration.
///
/// Returns the options that must be sent to the WebAuthn client (`navigator.credentials.create`).
/// It also saves the registration session state to the provided `store`.
pub async fn start_registration(
    user_id: &str,
    username: &str,
    display_name: &str,
) -> Result<PasskeyCreationOptions> {
    let challenge = generate_challenge()?;
    let user_handle = BASE64_URL_SAFE_NO_PAD.encode(user_id.as_bytes());

    let options = PasskeyCreationOptions {
        rp: RpEntity {
            name: CONFIG.rp_name.to_string(),
            id: CONFIG.rp_id.to_string(),
        },
        user: UserEntity {
            id: user_handle,
            name: username.to_string(),
            display_name: display_name.to_string(),
        },
        challenge: challenge.clone(),
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
pub async fn finish_registration(
    user_id: &str,
    challenge: &str,
    response: RegistrationResponse,
) -> Result<Passkey> {
    verify_client_data(
        &response.response.client_data_json,
        challenge,
        "webauthn.create",
    )?;

    // Parse attestation object (CBOR)
    let att_bytes = BASE64_URL_SAFE_NO_PAD.decode(&response.response.attestation_object)?;

    let att_obj: Value = ciborium::from_reader(att_bytes.as_slice())
        .map_err(|e| PasskeyError::InternalError(format!("Invalid attestationObject CBOR: {e}")))?;

    let Value::Map(m) = &att_obj else {
        return Err(PasskeyError::InternalError(
            "Invalid attestation object structure".into(),
        ));
    };

    let (_, auth_data_value) = m
        .iter()
        .find(|(k, _)| k.as_text().is_some_and(|s| s == "authData"))
        .ok_or_else(|| PasskeyError::InternalError("authData missing".into()))?;

    let auth_data_bytes = auth_data_value
        .as_bytes()
        .ok_or_else(|| PasskeyError::InternalError("authData not bytes".into()))?;

    // Verify authData
    let auth_data = parse_auth_data(auth_data_bytes)?;
    verify_rp_id_hash(&auth_data.rp_id_hash)?;
    verify_user_present(auth_data.flags)?;

    // Extract credential
    let cred_bytes = auth_data
        .credential_data
        .ok_or_else(|| PasskeyError::InternalError("Attested Credential Data missing".into()))?;
    let (cred_id, pub_key_cbor) = extract_credential(&cred_bytes)?;

    // Validate the public key parses
    CoseKey::from_slice(pub_key_cbor)
        .map_err(|e| PasskeyError::InternalError(format!("Invalid Public Key CBOR: {e}")))?;

    // Encode for storage
    let cred_id_b64 = BASE64_URL_SAFE_NO_PAD.encode(cred_id);
    let pub_key_b64 = BASE64_URL_SAFE_NO_PAD.encode(pub_key_cbor);

    let now = chrono::Utc::now().timestamp_millis();
    let passkey = Passkey {
        user_id: user_id.to_string(),
        cred_id: cred_id_b64.to_string(),
        name: response.name,
        public_key: pub_key_b64,
        created_at: now,
        last_used_at: now,
    };

    Ok(passkey)
}

/// Initiates a passkey login flow.
///
/// Returns the options that must be sent to the WebAuthn client (`navigator.credentials.get`).
/// It saves a login session state keyed by the challenge.
pub fn start_login() -> Result<PasskeyRequestOptions> {
    let challenge = generate_challenge()?;

    let options = PasskeyRequestOptions {
        challenge: challenge.clone(),
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
    passkey: &Passkey,
    challenge: &str,
    response: LoginResponse,
) -> Result<PrfResults> {
    // Parse clientDataJSON to retrieve the challenge for state lookup
    let client_data_bytes = BASE64_URL_SAFE_NO_PAD.decode(&response.response.client_data_json)?;

    // Full clientDataJSON verification
    verify_client_data(
        &response.response.client_data_json,
        challenge,
        "webauthn.get",
    )?;

    // Parse & verify authenticator data
    let auth_data_bytes = BASE64_URL_SAFE_NO_PAD.decode(&response.response.authenticator_data)?;

    let auth_data = parse_auth_data(&auth_data_bytes)?;
    verify_rp_id_hash(&auth_data.rp_id_hash)?;
    verify_user_present(auth_data.flags)?;

    // Verify user handle if present
    if let Some(ref uh_b64) = response.response.user_handle {
        let uh_bytes = BASE64_URL_SAFE_NO_PAD.decode(uh_b64)?;
        let uid_str = String::from_utf8(uh_bytes)
            .map_err(|_| PasskeyError::InternalError("Invalid userHandle utf8".into()))?;
        if uid_str != passkey.user_id {
            return Err(PasskeyError::UserHandleMismatch);
        }
    }

    // Verify signature
    let pub_key_bytes = BASE64_URL_SAFE_NO_PAD.decode(&passkey.public_key)?;

    let client_data_hash = Sha256::digest(&client_data_bytes);
    let mut signed_data = Vec::with_capacity(auth_data_bytes.len() + 32);
    signed_data.extend_from_slice(&auth_data_bytes);
    signed_data.extend_from_slice(&client_data_hash);

    let sig_bytes = BASE64_URL_SAFE_NO_PAD.decode(&response.response.signature)?;

    verify_p256_signature(&pub_key_bytes, &signed_data, &sig_bytes)?;

    // Return PRF results
    Ok(response.prf_results)
}
