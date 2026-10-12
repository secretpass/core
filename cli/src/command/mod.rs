mod instance;
mod logger;

use crate::project::get_working_directory;
pub use instance::CommandInstance;
use processkit::ProcessGroup;
use std::collections::HashMap;
use std::sync::OnceLock;
use tokio::sync::{Mutex, MutexGuard};
use uuid::Uuid;

pub struct InstanceManager {
    pub group: ProcessGroup,
    pub instances: HashMap<String, CommandInstance>,
}

static PROCESS_GROUP: OnceLock<Mutex<ProcessGroup>> = OnceLock::new();
static COMMAND_INSTANCES: OnceLock<Mutex<HashMap<String, CommandInstance>>> = OnceLock::new();

pub fn init_command_instances(
    workdir: Option<String>,
    command: Option<String>,
) -> anyhow::Result<()> {
    let group = ProcessGroup::new()?;
    let mut instances: HashMap<String, CommandInstance> = HashMap::new();
    let workdir = get_working_directory().command_working_dir(workdir)?;

    if let Some(command) = command {
        instances.insert(
            Uuid::new_v4().to_string(),
            CommandInstance::new(workdir, command),
        );
    }

    COMMAND_INSTANCES
        .set(Mutex::new(instances))
        .map_err(|_| anyhow::anyhow!("Command instances already initialized"))?;
    PROCESS_GROUP
        .set(Mutex::new(group))
        .map_err(|_| anyhow::anyhow!("Process group already initialized"))?;

    Ok(())
}

pub async fn get_command_instances() -> MutexGuard<'static, HashMap<String, CommandInstance>> {
    COMMAND_INSTANCES.get().unwrap().lock().await
}

pub async fn get_process_group() -> MutexGuard<'static, ProcessGroup> {
    PROCESS_GROUP.get().unwrap().lock().await
}
