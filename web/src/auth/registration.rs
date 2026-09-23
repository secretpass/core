use crate::auth::mapping::map_browser_passkey_registration_response;
use crate::passkeys::types::{PasskeyCreationOptions, RegistrationResponse};
use crate::passkeys::{finish_registration, start_registration};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use crate::auth::helpers::{get_credentials_container, parse_promise};
use secretpass_core::Passkey;
use web_sys::js_sys::{Array as JsArray, Object as JsObject, Reflect as JsReflect};
use web_sys::{
    AuthenticationExtensionsClientInputs, AuthenticationExtensionsPrfInputs,
    CredentialCreationOptions, PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity,
    PublicKeyCredentialUserEntity,
};

#[wasm_bindgen]
pub async fn register_user(
    user_id: String,
    email_address: String,
    display_name: String,
    key_name: String,
) -> Passkey {
    let options = start_registration(
        user_id.as_str(),
        email_address.as_str(),
        display_name.as_str(),
    )
    .await
    .expect_throw("Error starting passkey registration");
    let challenge = options.challenge.clone();

    let reg_response = browser_user_registration(key_name, options).await;

    let passkey = finish_registration(user_id.as_str(), challenge.as_str(), reg_response)
        .await
        .expect_throw("Error completing passkey registration");

    passkey
}

async fn browser_user_registration(
    key_name: String,
    create_options: PasskeyCreationOptions,
) -> RegistrationResponse {
    let rp = PublicKeyCredentialRpEntity::new(create_options.rp.name.as_str());
    let user_id = create_options.user.id.clone();
    let mut user_id_bytes: Vec<u8> = user_id.into_bytes();
    let user = PublicKeyCredentialUserEntity::new_with_u8_slice(
        create_options.user.name.as_str(),
        create_options.user.display_name.as_str(),
        &mut user_id_bytes,
    );

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
        &JsValue::from(-1),
    )
    .unwrap();

    let params = JsArray::new();
    params.push(&params_object);

    // Request PRF extension during key creation, this is a dealbreaker if not supported
    let extensions = AuthenticationExtensionsClientInputs::new();
    let prf_extension = AuthenticationExtensionsPrfInputs::new();
    extensions.set_prf(&prf_extension);

    rp.set_id(create_options.rp.id.as_str());
    let challenge = create_options.challenge.clone();
    let mut challenge_bytes: Vec<u8> = challenge.into_bytes();
    let pk_options = PublicKeyCredentialCreationOptions::new_with_u8_slice(
        &mut challenge_bytes,
        &params,
        &rp,
        &user,
    );

    pk_options.set_extensions(&extensions);

    let options = CredentialCreationOptions::new();
    options.set_public_key(&pk_options);

    let promise = get_credentials_container()
        .create_with_options(&options)
        .expect_throw("Error creating passkey");

    let credential = parse_promise(promise).await;

    map_browser_passkey_registration_response(key_name, credential)
}
