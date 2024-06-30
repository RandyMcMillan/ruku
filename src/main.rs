mod deploy;

use std::env;
use clap::{Parser, Subcommand};

use deploy::Deploy;

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

fn main() {
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
            println!("Detecting path...");
            let path = env::current_dir().unwrap_or_else(|e| {
                eprintln!("\n Ruku was unable to resolve the current directory path");
                std::process::exit(1);
            });
            let deploy = Deploy::new(path);
            deploy.run();
        }
        Commands::Destroy => {
            println!("Destroying application...");
        }
    }
}