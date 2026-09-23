use crate::auth::helpers::{get_credentials_container, parse_promise};
use crate::auth::mapping::{allowed_credentials, map_browser_passkey_authentication};
use crate::passkeys::types::{LoginResponse, PasskeyRequestOptions, PrfResults};
use crate::passkeys::{finish_login, start_login};
use crate::session::SecureSession;
use secretpass_core::{Passkey, SecretpassProject};
use sha2::{Digest, Sha256};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys::Uint8Array;
use web_sys::{
    AuthenticationExtensionsClientInputs, AuthenticationExtensionsPrfInputs,
    AuthenticationExtensionsPrfValues, CredentialRequestOptions, PublicKeyCredentialRequestOptions,
    UserVerificationRequirement,
};

#[wasm_bindgen]
pub async fn login_user(project: &SecretpassProject, passkey: Passkey) -> SecureSession {
    let login_options = start_login().expect_throw("Error starting login process");

    let challenge = login_options.challenge.clone();

    let (prf_results, login_response) = browser_user_login(project, &passkey, login_options).await;

    finish_login(&project, &passkey, challenge.as_str(), login_response)
        .await
        .expect_throw("Error finishing login process");

    SecureSession::new(project, passkey, prf_results)
}

async fn browser_user_login(
    project: &SecretpassProject,
    passkey: &Passkey,
    login_options: PasskeyRequestOptions,
) -> (PrfResults, LoginResponse) {
    let challenge = login_options.challenge.clone();
    let mut challenge_bytes: Vec<u8> = challenge.into_bytes();

    let prf_value =
        AuthenticationExtensionsPrfValues::new_with_u8_array(&encode_prf_salt(project.prf_salt()));
    prf_value.set_second_u8_array(&encode_prf_salt(passkey.prf_salt()));

    let prf_extension = AuthenticationExtensionsPrfInputs::new();
    prf_extension.set_eval(&prf_value);

    let extensions = AuthenticationExtensionsClientInputs::new();
    extensions.set_prf(&prf_extension);

    let pk_options = PublicKeyCredentialRequestOptions::new_with_u8_slice(&mut challenge_bytes);
    pk_options.set_rp_id(login_options.rp_id.as_str());
    pk_options.set_allow_credentials(&allowed_credentials(passkey));
    pk_options.set_user_verification(UserVerificationRequirement::Required);
    pk_options.set_extensions(&extensions);

    let options = CredentialRequestOptions::new();
    options.set_public_key(&pk_options);

    let promise = get_credentials_container()
        .get_with_options(&options)
        .expect_throw("Error getting the passkey");

    let credential = parse_promise(promise).await;

    map_browser_passkey_authentication(credential)
}

fn encode_prf_salt(value: String) -> Uint8Array {
    Uint8Array::new_from_slice(&Sha256::digest(value.as_bytes()))
}
