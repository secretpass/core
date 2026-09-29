use crate::api::config::config_routes;
use crate::api::utility::utilities_router;
use axum::Router;

pub async fn serve_local_server(cloud: bool, cloud_origin: String) {
    let app = Router::new()
        .merge(config_routes())
        .merge(utilities_router());

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
