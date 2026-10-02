use crate::api;
use tokio::task::JoinHandle;

pub fn start_manager(
    working_dir: Option<String>,
    is_cloud: bool,
    cloud_origin: String,
) -> anyhow::Result<JoinHandle<()>> {
    api::init_config(working_dir, is_cloud)?;

    // Launch the local API server
    Ok(tokio::spawn(api::serve_local_server(
        is_cloud,
        cloud_origin,
    )))
}
