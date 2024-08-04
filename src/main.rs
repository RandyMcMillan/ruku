use bollard::Docker;
use clap::{Parser, Subcommand};

use logger::Logger;
use server_config::ServerConfig;

use crate::git::Git;

mod container;
mod deploy;
mod git;
mod logger;
mod misc;
mod model;
mod server_config;

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
    /// Git hook
    #[command(name = "git-hook")]
    GitHook {
        /// The git repository name
        repo: String,
    },
    /// Git receive pack
    #[command(name = "git-receive-pack")]
    GitReceivePack {
        /// The git repository name
        repo: String,
    },
    /// Git upload pack
    #[command(name = "git-upload-pack")]
    GitUploadPack {
        /// The git repository name
        repo: String,
    },
}

#[tokio::main]
async fn main() {
    let log = Logger::default();
    log.section("... RUKU ...");

    let server_config = ServerConfig::new().unwrap_or_else(|e| {
        log.error(&format!("Error loading server config: {}", e));
        std::process::exit(1);
    });

    let git = Git::new(&log, &server_config);
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
        }
        Commands::Deploy => {
            log.section("Starting deployment");
        }
        Commands::Stop => {
            log.section("Stopping application...");
        }
        Commands::Destroy => {
            println!("Destroying application...");
        }
        Commands::GitHook { repo } => {
            log.step("Git hook");
            git.cmd_git_hook(repo);
        }
        Commands::GitReceivePack { repo } => {
            log.step("Git receive pack");
            git.cmd_git_receive_pack(repo);
        }
        Commands::GitUploadPack { repo } => {
            log.step("Git upload pack");
            git.cmd_git_upload_pack(repo);
        }
    }
}

async fn get_docker(log: &Logger) -> Docker {
    let docker = load_docker(log).await;

    let version = docker
        .version()
        .await
        .unwrap_or_else(|_| {
            log.error("Ruku was unable to connect to docker");
            std::process::exit(1);
        })
        .version
        .unwrap();
    log.step(&format!("Docker engine version: {}", version));

    docker
}

async fn load_docker(log: &Logger) -> Docker {
    Docker::connect_with_local_defaults().unwrap_or_else(|_| {
        log.error("Ruku was unable to connect to docker");
        std::process::exit(1);
    })
}
