use crate::api;
use crate::api::init_config;
use tokio::task::JoinHandle;

pub fn start_manager(
    working_dir: Option<String>,
    is_cloud: bool,
    cloud_origin: String,
) -> JoinHandle<()> {
    init_config(working_dir, is_cloud);

    // Launch the local API server
    tokio::spawn(api::serve_local_server(is_cloud, cloud_origin))
}
