use crate::manager::start_manager;
use clap::{Parser, Subcommand};

mod api;
mod manager;
mod project;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        long,
        global = true,
        default_value_t = false,
        help = "Use cloud instead of local storage"
    )]
    cloud: bool,

    #[arg(long, global = true, default_value = "https://secretpass.cloud")]
    cloud_origin: String,

    #[arg(long, global = true, default_value = None, help = "Project directory, defaults to current working directory")]
    project_dir: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Manager,
    Run {
        #[arg(long, default_value = None, help = "optional docker image to run; coming soon!")]
        image: Option<String>,

        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true,
            allow_negative_numbers = true,
            help = "Command to run"
        )]
        target: Vec<String>,
    },
    Direct {
        #[arg(long)]
        export: String, // dotenv, json
        #[arg(long)]
        private_key: Option<String>,
        #[arg(long)]
        env: Option<String>,
        #[arg(long)]
        secrets: Option<String>,
        #[arg(long, default_value = None, help = "optional docker image to run; coming soon!")]
        image: Option<String>,
        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true,
            allow_negative_numbers = true,
            help = "Command to run"
        )]
        target: Vec<String>,
    },
    Export,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Manager => {
            if cli.cloud {
                // Don't start the local manager when running in cloud mode
                println!(
                    "Please manage your secrets directly from: {}",
                    cli.cloud_origin.clone()
                );
                webbrowser::open(cli.cloud_origin.as_str()).unwrap();
                return;
            }

            start_manager(cli.project_dir, cli.cloud, cli.cloud_origin.clone())
                .unwrap()
                .await
                .unwrap();
        }
        Commands::Run { image, target } => {
            let future = start_manager(cli.project_dir, cli.cloud, cli.cloud_origin.clone());

            future.unwrap().await.unwrap();
        }
        Commands::Direct {
            export,
            private_key,
            env,
            secrets,
            image,
            target,
        } => {}
        Commands::Export => {}
    }
}
