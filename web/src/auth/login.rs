use crate::auth::helpers::{get_credentials_container, parse_promise};
use crate::auth::mapping::{
    allowed_credentials, build_credential_request_extensions, map_browser_passkey_authentication,
};
use crate::passkeys::types::{LoginResponse, PasskeyRequestOptions, PrfResults};
use crate::passkeys::{finish_login, start_login};
use crate::session::SecureSession;
use secretpass_core::{SecretpassProject, SecretpassUser, StoredPublicKey};
use serde::Deserialize;
use tsify::Tsify;
use web_sys::{
    CredentialRequestOptions, PublicKeyCredentialRequestOptions, UserVerificationRequirement,
};

#[derive(Debug, Deserialize, Tsify)]
pub struct LoginParams {
    pub(crate) project: SecretpassProject,
    pub(crate) user: SecretpassUser,
    pub(crate) public_keys: Vec<StoredPublicKey>,
}

pub async fn login_user_bounded(
    params: LoginParams,
) -> anyhow::Result<(StoredPublicKey, SecureSession)> {
    let login_options = start_login()?;

    let challenge = login_options.challenge.clone();

    let (public_key, prf_results, login_response) =
        browser_user_login(&params, login_options).await?;

    let passkey = public_key.passkey.clone().ok_or_else(|| {
        anyhow::format_err!(
            "Internal error: machine key({}) used for login",
            public_key.id
        )
    })?;

    finish_login(&params.project, &passkey, &challenge, login_response).await?;

    let session = SecureSession::new(params.project.algorithm, prf_results)?;

    Ok((public_key, session))
}

async fn browser_user_login(
    params: &LoginParams,
    login_options: PasskeyRequestOptions,
) -> anyhow::Result<(StoredPublicKey, PrfResults, LoginResponse)> {
    let mut challenge = login_options.challenge.clone();

    let extensions = build_credential_request_extensions(&params.project, &params.user);

    let pk_options = PublicKeyCredentialRequestOptions::new_with_u8_slice(&mut challenge);
    pk_options.set_rp_id(login_options.rp_id.as_str());
    pk_options.set_allow_credentials(&allowed_credentials(&params.public_keys)?);
    pk_options.set_user_verification(UserVerificationRequirement::Required);
    pk_options.set_extensions(&extensions);

    let options = CredentialRequestOptions::new();
    options.set_public_key(&pk_options);

    let promise = match get_credentials_container()?.get_with_options(&options) {
        Ok(promise) => promise,
        Err(err) => return Err(anyhow::format_err!("Error getting passkey: {:?}", err)),
    };

    let credential = parse_promise(promise).await?;

    map_browser_passkey_authentication(credential, &params.public_keys)
}
