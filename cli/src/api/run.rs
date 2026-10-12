use crate::command::{CommandInstance, get_command_instances};
use crate::project::get_working_directory;
use axum::Router;
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use secretpass_core::{CommandInstanceDetails, LogItem};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

async fn list_commands() -> Result<Json<Vec<CommandInstanceDetails>>, (StatusCode, String)> {
    let mut instances = get_command_instances().await;

    let mut output: Vec<CommandInstanceDetails> = Default::default();
    for (id, instance) in instances.iter_mut() {
        instance.update_status().map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error checking status for '{}': {}", instance.command, err),
            )
        })?;
        output.push(instance.details(id.clone()))
    }

    Ok(Json(output))
}

#[derive(Debug, Deserialize)]
struct RunCommandParams {
    command: String,
    environment: String,
    variables: HashMap<String, String>,
    working_dir: Option<String>,
}

async fn run_command(
    Json(params): Json<RunCommandParams>,
) -> Result<Json<CommandInstanceDetails>, (StatusCode, String)> {
    let id = Uuid::new_v4().to_string();

    let working_dir = get_working_directory()
        .command_working_dir(params.working_dir.clone())
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

    let mut instance = CommandInstance::new(working_dir, params.command);

    instance
        .run(params.environment, params.variables)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error running command '{}': {}", instance.command, err),
            )
        })?;

    let details = instance.details(id.clone());

    let mut instances = get_command_instances().await;
    instances.insert(id, instance);

    Ok(Json(details))
}

async fn tail_logs(
    Path((id, cursor)): Path<(String, usize)>,
) -> Result<Json<Vec<LogItem>>, (StatusCode, String)> {
    let mut instances = get_command_instances().await;
    let instance = instances.get_mut(&id).ok_or((
        StatusCode::NOT_FOUND,
        format!("Command instance with key ID '{}' not found", id),
    ))?;

    let logs = instance.get_logs(cursor).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "Error reading logs for command '{}': {}",
                instance.command, err
            ),
        )
    })?;

    Ok(Json(logs))
}

#[derive(Debug, Deserialize)]
struct StartCommandParams {
    environment: String,
    variables: HashMap<String, String>,
}

async fn start_command(
    Path(id): Path<String>,
    Json(params): Json<StartCommandParams>,
) -> Result<Json<CommandInstanceDetails>, (StatusCode, String)> {
    let mut instances = get_command_instances().await;
    let instance = instances.get_mut(&id).ok_or((
        StatusCode::NOT_FOUND,
        format!("Command instance with key ID '{}' not found", id),
    ))?;

    instance
        .run(params.environment, params.variables)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error running command: {}", err),
            )
        })?;

    Ok(Json(instance.details(id.clone())))
}

#[axum::debug_handler]
async fn stop_command(
    Path(id): Path<String>,
) -> Result<Json<CommandInstanceDetails>, (StatusCode, String)> {
    let mut instances = get_command_instances().await;
    let instance = instances.get_mut(&id).ok_or((
        StatusCode::NOT_FOUND,
        format!("Command instance with key ID '{}' not found", id),
    ))?;

    instance.stop(false).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to stop command: {}", err),
        )
    })?;

    let details = instance.details(id.clone());

    Ok(Json(details))
}

#[axum::debug_handler]
async fn dismiss_command(
    Path(id): Path<String>,
) -> Result<Json<CommandInstanceDetails>, (StatusCode, String)> {
    let mut instances = get_command_instances().await;
    let instance = instances.get_mut(&id).ok_or((
        StatusCode::NOT_FOUND,
        format!("Command instance with key ID 'id{}' not found", id),
    ))?;

    instance.stop(true).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to stop command: {}", err),
        )
    })?;

    let details = instance.details(id.clone());

    instances.remove(&id);

    Ok(Json(details))
}

pub fn command_run_routes() -> Router {
    Router::new()
        .route("/api/run/instances", get(list_commands))
        .route("/api/run/instance", post(run_command))
        .route("/api/run/instance/{id}", delete(dismiss_command))
        .route("/api/run/instance/{id}/logs/{cursor}", get(tail_logs))
        .route("/api/run/instance/{id}/start", post(start_command))
        .route("/api/run/instance/{id}/stop", post(stop_command))
}
