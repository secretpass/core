use crate::auth::mapping::map_browser_passkey_registration_response;
use crate::passkeys::types::{PasskeyCreationOptions, RegistrationResponse};
use crate::passkeys::{finish_registration, start_registration};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use crate::auth::helpers::{get_credentials_container, parse_promise};
use secretpass_core::{Passkey, PasskeyResidency, SecretpassProject};
use web_sys::js_sys::{Array as JsArray, Object as JsObject, Reflect as JsReflect};
use web_sys::{
    AttestationConveyancePreference, AuthenticationExtensionsClientInputs,
    AuthenticationExtensionsPrfInputs, AuthenticatorAttachment, AuthenticatorSelectionCriteria,
    CredentialCreationOptions, PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity, UserVerificationRequirement,
};

#[wasm_bindgen]
pub async fn register_user(
    project: &SecretpassProject,
    user_id: String,
    email_address: String,
    display_name: String,
) -> Passkey {
    let options = start_registration(
        project,
        user_id.as_str(),
        email_address.as_str(),
        display_name.as_str(),
    )
    .await
    .expect_throw("Error starting passkey registration");

    let reg_response = browser_user_registration(project, &options).await;

    finish_registration(project, &options, reg_response)
        .await
        .expect_throw("Error completing passkey registration")
}

async fn browser_user_registration(
    project: &SecretpassProject,
    create_options: &PasskeyCreationOptions,
) -> RegistrationResponse {
    let rp = PublicKeyCredentialRpEntity::new(create_options.rp.name.as_str());
    let user_id = create_options.user.id.clone();
    let mut user_id_bytes: Vec<u8> = user_id.into_bytes();
    let user = PublicKeyCredentialUserEntity::new_with_u8_slice(
        create_options.user.name.as_str(),
        create_options.user.display_name.as_str(),
        &mut user_id_bytes,
    );

    // Request PRF extension during key creation, this is a dealbreaker if not supported
    let extensions = AuthenticationExtensionsClientInputs::new();
    let prf_extension = AuthenticationExtensionsPrfInputs::new();
    extensions.set_prf(&prf_extension);

    rp.set_id(create_options.rp.id.as_str());
    let challenge = create_options.challenge.clone();
    let mut challenge_bytes: Vec<u8> = challenge.into_bytes();
    let pk_options = PublicKeyCredentialCreationOptions::new_with_u8_slice(
        &mut challenge_bytes,
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
    if project.residency == PasskeyResidency::HardwareKey {
        selection.set_authenticator_attachment(AuthenticatorAttachment::Platform);
    }

    // We will provide allowed credentials + username for every project, we don't need the key to be discoverable
    if project.residency == PasskeyResidency::OnDevice
        || project.residency == PasskeyResidency::HardwareKey
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

    let promise = get_credentials_container()
        .create_with_options(&options)
        .expect_throw("Error creating passkey");

    let credential = parse_promise(promise).await;

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
