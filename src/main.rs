mod deploy;
mod logger;

use std::env;
use std::path::Path;

use clap::{Parser, Subcommand};
use colored::Colorize;
use deploy::Deploy;
use dotenvy::dotenv;
use serde::Deserialize;
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
    /// Stop the application
    Stop,
    /// Deploy the application
    Deploy,
    /// Destroy the application
    Destroy,
}

#[derive(Deserialize, Debug)]
struct Config {
    port: u16,
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let log = Logger::new();
    log.step("Detecting path...", );

    let path = env::current_dir().unwrap_or_else(|_| {
        eprintln!("Ruku was unable to resolve the current directory path");
        std::process::exit(1);
    });

    // Check if a .env file exists in the current path
    let dotenv_path = Path::new(".env");
    if dotenv_path.exists() {
        dotenv().expect(".env file not found");
    }

    let config = envy::from_env::<Config>().unwrap_or_else(|_| {
        eprintln!("Ruku was unable to resolve the PORT environment variable");
        std::process::exit(1);
    });

    let app_name = config
        .name
        .unwrap_or_else(|| path.file_name().unwrap().to_str().unwrap().to_string());

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
                eprintln!("Invalid format. Use KEY=VALUE");
            }
        }
        Commands::ConfigGet { key } => {
            println!("Getting configuration for: {}", key);
        }
        Commands::Stop => {
            println!("Stopping application...");
        }
        Commands::Deploy => {
            let deploy = Deploy::new(log, app_name, path.display().to_string(), config.port);
            deploy.run().await;
        }
        Commands::Destroy => {
            println!("Destroying application...");
        }
    }
}
