use crate::api::{init_config, serve_local_server};
use crate::project::setup_working_directory;
use clap::Parser;

mod api;
pub mod command;
mod machine;
mod manager;
mod project;

use crate::command::init_command_instances;
use crate::machine::run_command;
use machine::export_secrets;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(
        long,
        global = true,
        default_value_t = false,
        help = "Use cloud instead of local storage"
    )]
    cloud: bool,

    #[arg(long, default_value = "https://secretpass.cloud")]
    cloud_origin: String,

    #[arg(
        long,
        global = true,
        default_value_t = false,
        help = "Disable builtin Posthog(EU) support chat features and activity tracking"
    )]
    no_support: bool,

    #[arg(long, default_value = None, help = "Project directory, defaults to current working directory")]
    project_dir: Option<String>,

    #[arg(long, default_value = None, help = "Directory to use as CWD, while running provided command")]
    command_dir: Option<String>,

    #[arg(long, default_value = None, help = "Export secrets as .env or json, requires a private key; pipe the results to the relevant file")]
    export: Option<String>, // .env, json

    #[arg(long, default_value = None, help = "Private key to use to run provided command or export secrets")]
    private_key: Option<String>,

    #[arg(long, default_value = None, help = "Secrets environment to use to run provided command or export secrets, required if private key has access to multiple environments")]
    env: Option<String>,

    #[arg(
        trailing_var_arg = true,
        allow_hyphen_values = true,
        allow_negative_numbers = true,
        help = "Command to run"
    )]
    target: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.cloud {
        println!("Cloud mode is not yet supported, WIP!");
        return;
    }

    setup_working_directory(args.project_dir);

    init_config(args.cloud, args.private_key.is_some())
        .map_err(|err| anyhow::format_err!("Error initializing from project config {}", err))
        .unwrap();

    if let Some(private_key) = args.private_key {
        if let Some(format) = args.export {
            if !args.target.is_empty() {
                panic!("You cannot export secrets and run a command at the same time");
            }
            export_secrets(private_key, args.env, format).unwrap();
            return;
        }

        if args.target.is_empty() {
            panic!("You must specify a command to run");
        }

        let command = args.target.join(" ");
        run_command(private_key, args.env, command, args.command_dir).unwrap();
    } else {
        if args.export.is_some() {
            panic!(
                "Exporting secrets not supported under non-human mode. Use the UI to export the secrets."
            );
        }

        let command = if args.target.is_empty() {
            None
        } else {
            Some(args.target.join(" "))
        };

        init_command_instances(args.command_dir, command).unwrap();
        tokio::spawn(serve_local_server(
            args.cloud,
            args.cloud_origin.clone(),
            args.no_support,
        ))
        .await
        .unwrap()
    }
}
