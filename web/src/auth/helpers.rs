use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::js_sys::Promise;
use web_sys::{CredentialsContainer, PublicKeyCredential};

pub fn get_credentials_container() -> CredentialsContainer {
    let window = web_sys::window().expect_throw("Please run Secretpass in a supported browser");
    let navigator = window.navigator();

    navigator.credentials()
}

pub async fn parse_promise(promise: Promise) -> PublicKeyCredential {
    let raw_result = promise
        .await
        .expect_throw("Error processing passkey authentication");

    raw_result
        .dyn_into::<PublicKeyCredential>()
        .expect("Error parsing passkey authentication result")
}
