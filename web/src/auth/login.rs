use crate::auth::helpers::{get_credentials_container, parse_promise};
use crate::auth::mapping::{map_browser_passkey_authentication, u8_array_to_js};
use crate::passkeys::types::{LoginResponse, Passkey, PasskeyRequestOptions};
use crate::passkeys::{finish_login, start_login};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{
    AuthenticationExtensionsClientInputs, AuthenticationExtensionsPrfInputs,
    AuthenticationExtensionsPrfValues, CredentialRequestOptions, PublicKeyCredentialRequestOptions,
    UserVerificationRequirement,
};

#[wasm_bindgen]
pub async fn login_user(passkey: &Passkey) {
    let login_options = start_login().expect_throw("Error starting login process");

    let challenge = login_options.challenge.clone();

    let login_response = browser_user_login(passkey, login_options).await;

    let prf_results = finish_login(&passkey, challenge.as_str(), login_response)
        .await
        .expect_throw("Error finishing login process");
    println!("PRF Results: {:?}", prf_results);
}

async fn browser_user_login(
    passkey: &Passkey,
    login_options: PasskeyRequestOptions,
) -> LoginResponse {
    let challenge = login_options.challenge.clone();
    let mut challenge_bytes: Vec<u8> = challenge.into_bytes();

    let prf_value = AuthenticationExtensionsPrfValues::new_with_u8_array(&u8_array_to_js(
        "TODO: Define First Salt".as_bytes(),
    ));
    prf_value.set_second_u8_array(&u8_array_to_js("TODO: Define Second Salt".as_bytes()));

    let prf_extension = AuthenticationExtensionsPrfInputs::new();
    prf_extension.set_eval(&prf_value);

    let extensions = AuthenticationExtensionsClientInputs::new();
    extensions.set_prf(&prf_extension);

    let pk_options = PublicKeyCredentialRequestOptions::new_with_u8_slice(&mut challenge_bytes);
    pk_options.set_rp_id(login_options.rp_id.as_str());
    pk_options.set_allow_credentials(&passkey.allowed_credentials());
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
