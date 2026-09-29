use crate::passkeys::types::{
    AssertionResponse, AttestationResponse, LoginResponse, PrfResults, RegistrationResponse,
};
use secretpass_core::Passkey;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt, throw_str};
use web_sys::js_sys::{
    Array as JsArray, ArrayBuffer, Object as JsObject, Reflect as JsReflect, Uint8Array,
};
use web_sys::{
    AuthenticationExtensionsClientOutputs, AuthenticatorAssertionResponse,
    AuthenticatorAttestationResponse, PublicKeyCredential,
};

fn prf_error(id: &str) -> String {
    format!(
        "PRF_ERROR<{id}>: Browser not supported! Pseudo Random Functions extension is missing or disabled"
    )
}

fn get_prf_value(outputs: AuthenticationExtensionsClientOutputs) -> JsValue {
    let prf_key = JsValue::from_str("prf");

    let prf_result = JsReflect::get(&outputs, &prf_key).expect_throw(prf_error("01").as_str());
    if prf_result.is_undefined() || prf_result.is_null() {
        throw_str(prf_error("02").as_str())
    }

    prf_result
}

fn assert_prf_enabled(outputs: AuthenticationExtensionsClientOutputs) {
    let prf_result = get_prf_value(outputs);

    let enabled = JsReflect::get(&prf_result, &JsValue::from_str("enabled"))
        .expect_throw(prf_error("03").as_str())
        .as_bool()
        .expect_throw(prf_error("04").as_str());

    if !enabled {
        throw_str(prf_error("05").as_str())
    }
}

pub fn map_browser_passkey_registration_response(
    pk_credential: PublicKeyCredential,
) -> RegistrationResponse {
    let ext_outputs = pk_credential.get_client_extension_results();
    assert_prf_enabled(ext_outputs);

    let attestation_response = pk_credential
        .response()
        .dyn_into::<AuthenticatorAttestationResponse>()
        .expect_throw("Invalid attestation response");

    let response = AttestationResponse {
        attestation_object: Uint8Array::new(&attestation_response.attestation_object()).to_vec(),
        client_data_json: Uint8Array::new(&attestation_response.client_data_json()).to_vec(),
    };

    RegistrationResponse {
        id: pk_credential.id(),
        type_: pk_credential.type_(),
        response,
    }
}

pub fn parse_prf_result(outputs: AuthenticationExtensionsClientOutputs) -> PrfResults {
    let prf_result = get_prf_value(outputs);

    let results = JsReflect::get(&prf_result, &JsValue::from_str("results"))
        .expect_throw(prf_error("06").as_str());

    let first = JsReflect::get(&results, &JsValue::from_str("first"))
        .expect_throw(prf_error("07").as_str());
    let second = JsReflect::get(&results, &JsValue::from_str("second"))
        .expect_throw(prf_error("08").as_str());

    let first = parse_prf_value(first, prf_error("09").as_str());
    let second = parse_prf_value(second, prf_error("10").as_str());

    PrfResults { first, second }
}

pub fn map_browser_passkey_authentication(
    pk_credential: PublicKeyCredential,
) -> (PrfResults, LoginResponse) {
    let ext_outputs = pk_credential.get_client_extension_results();
    let prf_results = parse_prf_result(ext_outputs);

    let assertion_response = pk_credential
        .response()
        .dyn_into::<AuthenticatorAssertionResponse>()
        .expect_throw("Error parsing assertion response");
    let user_handle = assertion_response.user_handle();

    let response = AssertionResponse {
        authenticator_data: Uint8Array::new(&assertion_response.authenticator_data()).to_vec(),
        client_data_json: Uint8Array::new(&assertion_response.client_data_json()).to_vec(),
        signature: Uint8Array::new(&assertion_response.signature()).to_vec(),
        user_handle: user_handle.map(|value| Uint8Array::new(&value).to_vec()),
    };

    let login_response = LoginResponse {
        id: pk_credential.id(),
        type_: pk_credential.type_(),
        response,
    };

    (prf_results, login_response)
}

pub fn parse_prf_value(buffer: JsValue, exception: &str) -> [u8; 32] {
    let buffer = ArrayBuffer::from(buffer);
    let as_vec = Uint8Array::new(&buffer).to_vec();
    as_vec.try_into().expect_throw(exception)
}

pub fn allowed_credentials(passkey: &Passkey) -> JsValue {
    let id = Uint8Array::new_from_slice(&passkey.id_bytes());

    let object = JsObject::new();
    JsReflect::set(
        &object,
        &JsValue::from_str("type"),
        &JsValue::from_str("public-key"),
    )
    .unwrap();
    JsReflect::set(&object, &JsValue::from_str("id"), &id).unwrap();

    let credentials = JsArray::new();
    credentials.push(&object);

    JsValue::from(credentials)
}
