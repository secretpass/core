use crate::api::project::project_routes;
use crate::api::run::command_run_routes;
use crate::api::secret::secrets_routes;
use crate::api::support::support_routes;
use crate::api::users::user_routes;
use crate::project::get_working_directory;
use axum::Router;
use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware;
use axum::response::Response;

pub async fn serve_local_server(cloud: bool, cloud_origin: String, no_support: bool) {
    let app = Router::new()
        .merge(project_routes())
        .merge(user_routes())
        .merge(secrets_routes())
        .merge(command_run_routes())
        .merge(support_routes(no_support))
        .layer(middleware::from_fn(add_dir_header));

    // Run the API on port 5000 proxied by vite while in debug mode
    let address = if cfg!(debug_assertions) {
        "secretpass.localhost:5000"
    } else {
        "secretpass.localhost:0"
    };

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let port = listener.local_addr().unwrap().port();
    println!(
        "Secret manager service listening on http://secretpass.localhost:{}",
        port
    );

    // Launch management interface
    // secretpass.localhost && secretpass.cloud/run/{port}
    let management_url = if cloud {
        format!("{}/run/{}", cloud_origin, port)
    } else {
        format!("http://secretpass.localhost:{}", port)
    };

    // Don't open the management interface in debug mode
    if !cfg!(debug_assertions) {
        let success = webbrowser::open(&management_url).is_ok();
        if !success {
            println!("Failed to open, please open {} to proceed", management_url);
        }
    }

    axum::serve(listener, app).await.unwrap();
}

async fn add_dir_header(req: Request, next: middleware::Next) -> Response {
    let mut response = next.run(req).await;

    let dir = get_working_directory();

    response.headers_mut().insert(
        "x-current-working-directory",
        HeaderValue::from_str(dir.short_path().as_str()).unwrap(),
    );

    response
}
