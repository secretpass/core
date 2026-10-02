use crate::auth::helpers::{get_credentials_container, parse_promise};
use crate::auth::mapping::{
    build_credential_request_extensions, map_browser_passkey_registration_response,
};
use crate::passkeys::types::{PasskeyCreationOptions, PrfResults, RegistrationResponse};
use crate::passkeys::{finish_registration, start_registration};
use crate::session::SecureSession;
use secretpass_core::{PasskeyResidency, SecretpassProject, SecretpassUser, StoredPublicKey};
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Array as JsArray, Object as JsObject, Reflect as JsReflect};
use web_sys::{
    AttestationConveyancePreference, AuthenticatorAttachment, AuthenticatorSelectionCriteria,
    CredentialCreationOptions, PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity, UserVerificationRequirement,
};

#[derive(Debug, Deserialize, Tsify)]
pub struct RegistrationParams {
    name: String,
    project: SecretpassProject,
    user: SecretpassUser,
}

pub async fn register_user_bounded(params: RegistrationParams) -> anyhow::Result<StoredPublicKey> {
    let options = start_registration(&params.project, &params.user).await?;

    let (prf_results, reg_response) = browser_user_registration(&params, &options).await?;

    let passkey = finish_registration(&params.project, &options, reg_response)?;

    let session = SecureSession::new(params.project.algorithm, prf_results)?;

    Ok(session.build_public_key(params.name, &passkey))
}

async fn browser_user_registration(
    params: &RegistrationParams,
    create_options: &PasskeyCreationOptions,
) -> anyhow::Result<(PrfResults, RegistrationResponse)> {
    let rp = PublicKeyCredentialRpEntity::new(&create_options.rp.name);
    let user_id = create_options.user.id.clone();
    let mut user_id_bytes: Vec<u8> = user_id.into_bytes();
    let user = PublicKeyCredentialUserEntity::new_with_u8_slice(
        &create_options.user.name,
        &create_options.user.display_name,
        &mut user_id_bytes,
    );

    // Request PRF extension during key creation, this is a dealbreaker if not supported
    let extensions = build_credential_request_extensions(&params.project, &params.user);

    rp.set_id(&create_options.rp.id);
    let mut challenge = create_options.challenge.clone();
    let pk_options = PublicKeyCredentialCreationOptions::new_with_u8_slice(
        &mut challenge,
        &build_credential_params(),
        &rp,
        &user,
    );

    // Allow the user up to 60 seconds to complete the registration
    pk_options.set_timeout(60_000);
    pk_options.set_extensions(&extensions);
    pk_options.set_attestation(AttestationConveyancePreference::Direct);

    let selection = AuthenticatorSelectionCriteria::new();
    selection.set_user_verification(UserVerificationRequirement::Required);
    if params.project.residency == PasskeyResidency::HardwareKey {
        selection.set_authenticator_attachment(AuthenticatorAttachment::Platform);
    }

    // We will provide allowed credentials + username for every project, we don't need the key to be discoverable
    if params.project.residency == PasskeyResidency::OnDevice
        || params.project.residency == PasskeyResidency::HardwareKey
    {
        selection.set_require_resident_key(true);
        selection.set_resident_key("required");
    } else {
        selection.set_require_resident_key(false);
        selection.set_resident_key("discouraged");
    }

    pk_options.set_authenticator_selection(&selection);

    let options = CredentialCreationOptions::new();
    options.set_public_key(&pk_options);

    let promise = get_credentials_container()?
        .create_with_options(&options)
        .map_err(|_| anyhow::anyhow!("Error creating passkey"))?;

    let credential = parse_promise(promise).await?;

    map_browser_passkey_registration_response(credential)
}

fn build_credential_params() -> JsArray {
    let params = JsArray::new();
    // -8: Highly secure EdDSA (Ed25519)
    // -7: Universally supported ES256 (P-256)
    for algorithm in [-8, -7] {
        let params_object = JsObject::new();
        JsReflect::set(
            &params_object,
            &JsValue::from_str("type"),
            &JsValue::from_str("public-key"),
        )
        .unwrap();
        JsReflect::set(
            &params_object,
            &JsValue::from_str("alg"),
            &JsValue::from(algorithm),
        )
        .unwrap();
        params.push(&params_object);
    }
    params
}
