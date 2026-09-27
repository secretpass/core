use wasm_bindgen::{JsCast, UnwrapThrowExt, throw_str};
use web_sys::js_sys::Promise;
use web_sys::{CredentialsContainer, PublicKeyCredential};

pub fn get_credentials_container() -> CredentialsContainer {
    let window = web_sys::window().expect_throw("Please run Secretpass in a supported browser");
    let navigator = window.navigator();

    navigator.credentials()
}

pub async fn parse_promise(promise: Promise) -> PublicKeyCredential {
    let then = promise.await;
    match then {
        Ok(raw_result) => raw_result
            .dyn_into::<PublicKeyCredential>()
            .expect_throw("Error parsing passkey authentication result"),
        Err(err) => {
            throw_str(format!("Error processing passkey authentication: {:?}", err).as_str())
        }
    }
}
