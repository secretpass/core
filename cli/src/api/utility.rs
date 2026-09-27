use crate::project::get_working_directory;
use axum::Router;
use axum::routing::get;

async fn current_directory() -> String {
    get_working_directory().short_path()
}

pub fn utilities_router() -> Router {
    Router::new().route("/api/utils/cwd", get(current_directory))
}
