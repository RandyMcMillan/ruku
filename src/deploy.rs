use std::path::{Path, PathBuf};

use dotenvy::dotenv;
use nixpacks::create_docker_image;
use nixpacks::nixpacks::builder::docker::DockerBuilderOptions;
use nixpacks::nixpacks::plan::generator::GeneratePlanOptions;
use nixpacks::nixpacks::plan::BuildPlan;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    port: u16,
    name: Option<String>,
}

pub struct Deploy {
    pub path: PathBuf,
}

impl Deploy {
    pub fn new(path: PathBuf) -> Deploy {
        Deploy { path }
    }

    pub async fn run(&self) {
        println!("Deploying from {}", self.path.display());

        // Check if a .env file exists in the current path
        let dotenv_path = Path::new(".env");
        if dotenv_path.exists() {
            dotenv().expect(".env file not found");
        }

        let config = envy::from_env::<Config>().unwrap_or_else(|_| {
            eprintln!("\n Ruku was unable to resolve the PORT environment variable");
            std::process::exit(1);
        });

        let mut app_name = config.name;
        if app_name.is_none() {
            app_name = Option::from(self.path.file_name().unwrap().to_str().unwrap().to_string())
        }

        // Nix pack
        let env: Vec<&str> = vec![];
        let cli_plan = BuildPlan::default();
        let options = GeneratePlanOptions {
            plan: Some(cli_plan),
            config_file: None,
        };
        let build_options = &DockerBuilderOptions {
            name: app_name,
            out_dir: None,
            print_dockerfile: false,
            tags: vec![],
            labels: vec![],
            quiet: false,
            cache_key: None,
            no_cache: false,
            inline_cache: false,
            cache_from: None,
            platform: vec![],
            current_dir: true,
            no_error_without_start: false,
            incremental_cache_image: None,
            cpu_quota: None,
            memory: None,
            verbose: false,
            docker_host: None,
            docker_tls_verify: None,
        };
        create_docker_image(
            &self.path.display().to_string(),
            env,
            &options,
            build_options,
        )
        .await?;
    }
}
