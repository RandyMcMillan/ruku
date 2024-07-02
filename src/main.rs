mod container;
mod deploy;
mod logger;
mod misc;
mod model;

use std::env;
use std::path::Path;

use bollard::Docker;
use clap::{Parser, Subcommand};
use deploy::Deploy;
use dotenvy::dotenv;
use logger::Logger;

#[derive(Parser)]
#[command(about = "A CLI app for managing your server.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show logs
    Logs,
    /// Set a configuration variable, e.g, VAR=12
    #[command(name = "config:set")]
    ConfigSet {
        /// The configuration variable in the form KEY=VALUE
        var: String,
    },
    /// Get a configuration variable
    #[command(name = "config:get")]
    ConfigGet {
        /// The configuration variable name
        key: String,
    },
    /// Run the application
    Run,
    /// Deploy the application
    Deploy,
    /// Stop the application
    Stop,
    /// Destroy the application
    Destroy,
}

#[tokio::main]
async fn main() {
    let log = Logger::default();
    log.step("Detecting path...");

    let path = env::current_dir().unwrap_or_else(|_| {
        log.error("Ruku was unable to resolve the current directory path");
        std::process::exit(1);
    });

    log.step("Checking if docker is running...");
    let docker = load_docker(&log).await;
    let version = docker
        .version()
        .await
        .unwrap_or_else(|_| {
            log.error("Ruku was unable to connect to docker");
            std::process::exit(1);
        })
        .version
        .unwrap();
    log.step(format!("Docker engine version: {}", version).as_str());

    // Check if a .env file exists in the current path
    let dotenv_path = Path::new(".env");
    if dotenv_path.exists() {
        dotenv().expect(".env file not found");
    }

    let config = envy::from_env::<model::Config>().unwrap_or_else(|_| {
        log.error("Ruku was unable to resolve the PORT environment variable");
        std::process::exit(1);
    });

    let app_name = config
        .name
        .as_deref()
        .unwrap_or_else(|| path.file_name().unwrap().to_str().unwrap());

    let deploy = Deploy::new(&log, app_name, path.as_path().to_str().unwrap(), &config);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Logs => {
            println!("Showing logs...");
        }
        Commands::ConfigSet { var } => {
            println!("Setting configuration: {}", var);
            // Parse `var` into key and value
            let parts: Vec<&str> = var.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0];
                let value = parts[1];
                println!("Setting {} to {}", key, value);
            } else {
                log.error("Invalid format. Use KEY=VALUE");
            }
        }
        Commands::ConfigGet { key } => {
            println!("Getting configuration for: {}", key);
        }
        Commands::Run => {
            log.section("Running application");
            deploy.run().await;
        }
        Commands::Deploy => {
            log.section("Starting deployment");
            deploy.run().await;
        }
        Commands::Stop => {
            println!("Stopping application...");
        }
        Commands::Destroy => {
            println!("Destroying application...");
        }
    }
}

async fn load_docker(log: &Logger) -> Docker {
    Docker::connect_with_local_defaults().unwrap_or_else(|_| {
        log.error("Ruku was unable to connect to docker");
        std::process::exit(1);
    })
}
