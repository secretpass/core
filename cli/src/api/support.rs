use axum::Json;
use axum::http::StatusCode;
use serde::Serialize;

const POSTHOG_UI_HOST: &str = env!("POSTHOG_UI_HOST");
const POSTHOG_API_HOST: &str = env!("POSTHOG_API_HOST");
const POSTHOG_DEFAULTS: &str = env!("POSTHOG_DEFAULTS");
const POSTHOG_PROJECT_TOKEN: &str = env!("POSTHOG_PROJECT_TOKEN");

#[derive(Debug, Serialize)]
struct SupportOptions {
    api_host: String,
    ui_host: String,
    defaults: String,
    project_token: String,
}

async fn get_support_options() -> Result<Json<SupportOptions>, StatusCode> {
    Ok(Json(SupportOptions {
        ui_host: POSTHOG_UI_HOST.to_string(),
        api_host: POSTHOG_API_HOST.to_string(),
        defaults: POSTHOG_DEFAULTS.to_string(),
        project_token: POSTHOG_PROJECT_TOKEN.to_string(),
    }))
}

pub fn support_routes(no_support: bool) -> axum::Router {
    if no_support {
        return axum::Router::new();
    }

    axum::Router::new().route("/api/support", axum::routing::get(get_support_options))
}
