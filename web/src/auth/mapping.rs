use crate::passkeys::types::{
    AssertionResponse, AttestationResponse, LoginResponse, PrfResults, RegistrationResponse,
};
use secretpass_core::{SecretpassProject, SecretpassUser, StoredPublicKey};
use sha2::{Digest, Sha256};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{
    Array as JsArray, ArrayBuffer, Object as JsObject, Reflect as JsReflect, Uint8Array,
};
use web_sys::{
    AuthenticationExtensionsClientInputs, AuthenticationExtensionsClientOutputs,
    AuthenticationExtensionsPrfInputs, AuthenticationExtensionsPrfValues,
    AuthenticatorAssertionResponse, AuthenticatorAttestationResponse, PublicKeyCredential,
};

fn prf_error(id: &str) -> anyhow::Error {
    anyhow::format_err!(
        "PRF_ERROR<{id}>: Browser not supported! Pseudo Random Functions extension is missing or disabled"
    )
}

fn get_prf_value(outputs: AuthenticationExtensionsClientOutputs) -> anyhow::Result<JsValue> {
    let prf_key = JsValue::from_str("prf");

    let prf_result = match JsReflect::get(&outputs, &prf_key) {
        Ok(prf_result) => prf_result,
        Err(_) => return Err(prf_error("01")),
    };
    if prf_result.is_undefined() || prf_result.is_null() {
        return Err(prf_error("02"));
    }

    Ok(prf_result)
}

pub fn map_browser_passkey_registration_response(
    pk_credential: PublicKeyCredential,
) -> anyhow::Result<(PrfResults, RegistrationResponse)> {
    let ext_outputs = pk_credential.get_client_extension_results();
    let prf_result = parse_prf_result(ext_outputs)?;

    let attestation_response = match pk_credential
        .response()
        .dyn_into::<AuthenticatorAttestationResponse>()
    {
        Ok(attestation_response) => attestation_response,
        Err(_) => return Err(anyhow::anyhow!("Error parsing attestation response")),
    };

    let attestation_response = AttestationResponse {
        attestation_object: Uint8Array::new(&attestation_response.attestation_object()).to_vec(),
        client_data_json: Uint8Array::new(&attestation_response.client_data_json()).to_vec(),
    };

    let response = RegistrationResponse {
        id: pk_credential.id(),
        type_: pk_credential.type_(),
        response: attestation_response,
    };

    Ok((prf_result, response))
}

pub fn parse_prf_result(
    outputs: AuthenticationExtensionsClientOutputs,
) -> anyhow::Result<PrfResults> {
    let prf_result = get_prf_value(outputs)?;

    let results = match JsReflect::get(&prf_result, &JsValue::from_str("results")) {
        Ok(value) => value,
        Err(_) => return Err(prf_error("06")),
    };

    let first = match JsReflect::get(&results, &JsValue::from_str("first")) {
        Ok(value) => value,
        Err(_) => return Err(prf_error("07")),
    };
    let second = match JsReflect::get(&results, &JsValue::from_str("second")) {
        Ok(value) => value,
        Err(_) => return Err(prf_error("08")),
    };

    let first = parse_prf_value(first)?;
    let second = parse_prf_value(second)?;

    Ok(PrfResults { first, second })
}

pub fn map_browser_passkey_authentication(
    pk_credential: PublicKeyCredential,
    public_keys: &[StoredPublicKey],
) -> anyhow::Result<(StoredPublicKey, PrfResults, LoginResponse)> {
    let public_key = public_keys
        .iter()
        .find(|pk| pk.passkey.clone().unwrap().id == pk_credential.id())
        .ok_or_else(|| anyhow::anyhow!("Passkey did not match known passkeys"))?;

    let ext_outputs = pk_credential.get_client_extension_results();
    let prf_results = parse_prf_result(ext_outputs)?;

    let assertion_response = match pk_credential
        .response()
        .dyn_into::<AuthenticatorAssertionResponse>()
    {
        Ok(value) => value,
        Err(_) => return Err(anyhow::anyhow!("Error parsing assertion response")),
    };
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

    Ok((public_key.clone(), prf_results, login_response))
}

pub fn parse_prf_value(buffer: JsValue) -> anyhow::Result<[u8; 32]> {
    let buffer = ArrayBuffer::from(buffer);
    let as_vec = Uint8Array::new(&buffer).to_vec();
    let value: [u8; 32] = match as_vec.try_into() {
        Ok(value) => value,
        Err(_) => return Err(anyhow::anyhow!("Error parsing PRF value")),
    };
    Ok(value)
}

pub fn allowed_credentials(public_keys: &[StoredPublicKey]) -> anyhow::Result<JsValue> {
    let credentials = JsArray::new();
    for pk in public_keys {
        let passkey = pk
            .passkey
            .clone()
            .ok_or_else(|| anyhow::format_err!("Missing passkey in public key {}", pk.id))?;
        let id = Uint8Array::new_from_slice(&passkey.id_bytes()?);

        let object = JsObject::new();
        JsReflect::set(
            &object,
            &JsValue::from_str("type"),
            &JsValue::from_str("public-key"),
        )
        .unwrap();
        JsReflect::set(&object, &JsValue::from_str("id"), &id).unwrap();

        credentials.push(&object);
    }

    Ok(JsValue::from(credentials))
}

fn encode_prf_salt(value: String) -> Uint8Array {
    Uint8Array::new_from_slice(&Sha256::digest(value.as_bytes()))
}

pub fn build_credential_request_extensions(
    project: &SecretpassProject,
    user: &SecretpassUser,
) -> AuthenticationExtensionsClientInputs {
    let prf_value =
        AuthenticationExtensionsPrfValues::new_with_u8_array(&encode_prf_salt(project.prf_salt()));
    prf_value.set_second_u8_array(&encode_prf_salt(user.prf_salt()));

    let prf_extension = AuthenticationExtensionsPrfInputs::new();
    prf_extension.set_eval(&prf_value);

    let extensions = AuthenticationExtensionsClientInputs::new();
    extensions.set_prf(&prf_extension);

    extensions
}
