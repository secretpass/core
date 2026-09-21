use crate::passkeys::types::{AttestationResponse, RegistrationResponse};
use std::string::FromUtf8Error;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt, throw_str};
use web_sys::js_sys::{ArrayBuffer, Reflect, Uint8Array};
use web_sys::{AuthenticatorAttestationResponse, PublicKeyCredential};

fn prf_error(id: &str) -> String {
    format!(
        "PRF_ERROR<{id}>: Browser not supported! Pseudo Random Functions extension is missing or disabled"
    )
}

pub fn map_browser_passkey_registration_response(
    name: String,
    pk_credential: PublicKeyCredential,
) -> RegistrationResponse {
    let id = pk_credential.id();
    let type_ = pk_credential.type_();
    let raw_id = js_array_buffer_to_string(pk_credential.raw_id())
        .expect_throw("Error parsing passkey raw ID");

    let response = pk_credential.response();
    let attestation_response = response
        .dyn_into::<AuthenticatorAttestationResponse>()
        .expect_throw("Invalid attestation response");

    let client_data_json = js_array_buffer_to_string(attestation_response.client_data_json())
        .expect_throw("Error parsing passkey client data");
    let attestation_object = js_array_buffer_to_string(attestation_response.attestation_object())
        .expect_throw("Error parsing passkey attestation object");

    let response = AttestationResponse {
        attestation_object,
        client_data_json,
    };

    let ext_outputs = pk_credential.get_client_extension_results();

    let prf_key = JsValue::from_str("prf");

    let prf_object = Reflect::get(&ext_outputs, &prf_key).expect_throw(prf_error("01").as_str());
    if prf_object.is_undefined() || prf_object.is_null() {
        throw_str(prf_error("02").as_str())
    }

    let enabled_key = JsValue::from_str("enabled");
    let enabled = Reflect::get(&prf_object, &enabled_key)
        .expect_throw(prf_error("03").as_str())
        .as_bool()
        .expect_throw(prf_error("04").as_str());
    if !enabled {
        throw_str(prf_error("05").as_str())
    }

    let js_value: &JsValue = ext_outputs.as_ref();

    let extensions: serde_json::Value = serde_wasm_bindgen::from_value(js_value.clone())
        .expect_throw("Error processing extension outputs");

    RegistrationResponse {
        id,
        raw_id,
        type_,
        name: Some(name),
        response,
        client_extension_results: Some(extensions),
    }
}

pub fn js_array_buffer_to_vec(buffer: ArrayBuffer) -> Vec<u8> {
    Uint8Array::new(&buffer).to_vec()
}

pub fn js_array_buffer_to_string(buffer: ArrayBuffer) -> Result<String, FromUtf8Error> {
    String::from_utf8(Uint8Array::new(&buffer).to_vec())
}
