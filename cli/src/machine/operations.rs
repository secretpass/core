use crate::command::CommandInstance;
use crate::manager::get_readonly_manager;
use crate::project::{get_working_directory, read_secrets_value};
use secretpass_core::{PrivateKey, PrivateKeySeed, PublicKeyType};
use std::collections::HashMap;

fn decrypted_secrets(
    private_key: String,
    env_name: Option<String>,
) -> anyhow::Result<(String, HashMap<String, String>)> {
    let manager = get_readonly_manager().map_err(|(_, message)| anyhow::anyhow!(message))?;
    let seed = PrivateKeySeed::from_machine_key(manager.config.project.algorithm, private_key)?;

    let private_key = PrivateKey::from_seed(seed)?;
    let public_key = private_key.public_key();

    let key_id = public_key.calculate_id(PublicKeyType::Machine);

    manager
        .get_public_key(&key_id)
        .ok_or_else(|| anyhow::anyhow!("Public key not found"))?;

    let machine = manager
        .get_user_by_id(&key_id)
        .ok_or_else(|| anyhow::anyhow!("Machine access not defined"))?;

    if machine.access.is_empty() {
        return Err(anyhow::anyhow!("Key has no access to any environment"));
    }
    if env_name.is_none() && machine.access.len() != 1 {
        return Err(anyhow::anyhow!(
            "Key has access to multiple environments, please specify --env"
        ));
    }

    let env_name = env_name.unwrap_or(machine.access[0].clone());
    let environment = manager
        .get_environment(&env_name)
        .ok_or_else(|| anyhow::format_err!("Environment {} not found", env_name))?;

    let mut secrets: HashMap<String, String> = HashMap::new();
    for secret in environment.secrets.iter() {
        let name = secret.name.clone();
        let all_values = read_secrets_value(&env_name, &name)?
            .ok_or_else(|| anyhow::format_err!("Secret {} not found", &name))?;
        let encrypted_value = all_values
            .get(&key_id)
            .ok_or_else(|| anyhow::format_err!("Secret {} not encrypted for this key", &name))?;

        let secret_bytes = private_key.decrypt(encrypted_value)?;
        let plain_value = String::from_utf8(secret_bytes)?;

        secrets.insert(secret.name.clone(), plain_value);
    }

    Ok((env_name, secrets))
}

pub fn export_secrets(
    private_key: String,
    env_name: Option<String>,
    format: String,
) -> anyhow::Result<()> {
    let format = format.to_ascii_lowercase();
    let (_, secrets) = decrypted_secrets(private_key, env_name)?;

    if format == "json" {
        let json = serde_json::to_string_pretty(&secrets)?;
        println!("{}", json);
    } else if format == ".env" || format == "dotenv" {
        for (name, value) in secrets.iter() {
            println!("{}=\"{}\"", name, serde_json::to_string(&value)?);
        }
    } else {
        return Err(anyhow::format_err!("Unsupported format: {}", format));
    }

    Ok(())
}

pub fn run_command(
    private_key: String,
    env_name: Option<String>,
    command: String,
    workdir: Option<String>,
) -> anyhow::Result<()> {
    let (_, secrets) = decrypted_secrets(private_key, env_name)?;

    let workdir = get_working_directory().command_working_dir(workdir)?;

    let mut instance = CommandInstance::new(workdir, command);

    instance.run_machine(secrets);

    Ok(())
}
