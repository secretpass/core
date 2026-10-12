use crate::project::get_working_directory;
use std::collections::HashMap;
use std::fs;

pub fn write_secret_value(
    env_name: &str,
    secret_name: &str,
    value: HashMap<String, String>,
) -> anyhow::Result<()> {
    let dir = get_working_directory();
    let path = dir.secrets_path(env_name, secret_name);

    fs::create_dir_all(path.parent().unwrap())?;

    let content = serde_json::to_string_pretty(&value)?;
    fs::write(path, content)?;

    Ok(())
}

pub fn delete_secret(env_name: &str, secret_name: &str) -> anyhow::Result<()> {
    let dir = get_working_directory();
    let path = dir.secrets_path(env_name, secret_name);

    fs::remove_file(path)?;
    Ok(())
}

pub fn read_secrets_value(
    env_name: &str,
    secret_name: &str,
) -> anyhow::Result<Option<HashMap<String, String>>> {
    let dir = get_working_directory();

    let path = dir.secrets_path(env_name, secret_name);
    if !path.exists() {
        return Ok(None);
    }

    if !path.is_file() {
        return Err(anyhow::format_err!(
            "Path({}) is not a file",
            path.display()
        ));
    }

    let content = fs::read_to_string(path)?;
    let content: HashMap<String, String> = serde_json::from_str(&content)?;

    Ok(Some(content))
}
