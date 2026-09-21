use crate::auth::mapping::map_browser_passkey_registration_response;
use crate::constants::{WEBAUTH_RP_ENTITY, WEBAUTH_RP_ID, WEBAUTH_RP_ORIGIN};
use crate::passkeys::types::{
    PublicKeyCredentialCreationOptions as ServerPublicKeyCredentialCreationOptions,
    RegistrationResponse,
};
use crate::passkeys::{PasskeyConfig, finish_registration, start_registration};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{
    AuthenticationExtensionsClientInputs, AuthenticationExtensionsPrfInputs,
    CredentialCreationOptions, PublicKeyCredential, PublicKeyCredentialCreationOptions,
    PublicKeyCredentialRpEntity, PublicKeyCredentialUserEntity,
};

#[wasm_bindgen]
pub enum WebAuthState {
    Unauthorized,
    Registering,
    Registered,
    Authorizing,
    Authorized,
}

#[wasm_bindgen]
pub struct WebAuthSession {
    config: PasskeyConfig,
}

impl WebAuthSession {
    pub fn new() -> WebAuthSession {
        let rp_id = WEBAUTH_RP_ID.expect_throw("Secretpass was not compiled correctly");
        let rp_origin = WEBAUTH_RP_ORIGIN.expect_throw("Secretpass was not compiled correctly");
        let rp_name = WEBAUTH_RP_ENTITY.expect_throw("Secretpass was not compiled correctly");

        let config = PasskeyConfig {
            rp_id: rp_id.to_string(),
            rp_name: rp_name.to_string(),
            origin: rp_origin.to_string(),
            state_ttl: 300,
        };

        Self { config }
    }

    pub async fn perform_user_registration(
        &mut self,
        user_id: String,
        email_address: String,
        display_name: String,
        key_name: String,
    ) {
        let options = start_registration(
            user_id.as_str(),
            email_address.as_str(),
            display_name.as_str(),
            &self.config,
        )
        .await
        .expect_throw("Error starting passkey registration");
        let challenge = options.challenge.clone();

        let reg_response = self.perform_browser_registration(key_name, options).await;

        finish_registration(
            user_id.as_str(),
            challenge.as_str(),
            &self.config,
            reg_response,
        )
        .await
        .expect_throw("Error completing passkey registration");
    }

    async fn perform_browser_registration(
        &self,
        key_name: String,
        remote_options: ServerPublicKeyCredentialCreationOptions,
    ) -> RegistrationResponse {
        let window = web_sys::window().expect_throw("Please run Secretpass in a supported browser");

        let navigator = window.navigator();

        let credentials_container = navigator.credentials();

        let rp = PublicKeyCredentialRpEntity::new(remote_options.rp.name.as_str());
        let user_id = remote_options.user.id.clone();
        let mut user_id_bytes: Vec<u8> = user_id.into_bytes();
        let user = PublicKeyCredentialUserEntity::new_with_u8_slice(
            remote_options.user.name.as_str(),
            remote_options.user.display_name.as_str(),
            &mut user_id_bytes,
        );

        let params = serde_json::json!([
            {
                "type": "public-key",
                "alg": -7
            }
        ]);
        let params = serde_wasm_bindgen::to_value(&params)
            .expect_throw("Internal Error: Invalid registration params value");

        // Request PRF extension during key creation, this is a dealbreaker if not supported
        let extensions = AuthenticationExtensionsClientInputs::new();
        let prf_extension = AuthenticationExtensionsPrfInputs::new();
        extensions.set_prf(&prf_extension);

        rp.set_id(remote_options.rp.id.as_str());
        let challenge = remote_options.challenge.clone();
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

        let promise = credentials_container
            .create_with_options(&options)
            .expect_throw("Error creating passkey");

        let raw_result = promise
            .await
            .expect_throw("Error processing passkey authentication");

        let credential: PublicKeyCredential = raw_result
            .dyn_into::<PublicKeyCredential>()
            .expect("Error parsing passkey authentication result");

        map_browser_passkey_registration_response(key_name, credential)
    }
}
