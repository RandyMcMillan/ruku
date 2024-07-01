use nixpacks::create_docker_image;
use nixpacks::nixpacks::builder::docker::DockerBuilderOptions;
use nixpacks::nixpacks::plan::{generator::GeneratePlanOptions, BuildPlan};

use crate::logger::Logger;
use crate::model::Config;

pub struct Deploy<'a> {
    log: &'a Logger,

    name: String,
    path: String,
    config: Config,
}

impl<'a> Deploy<'a> {
    pub fn new(log: &Logger, name: String, path: String, config: Config) -> Deploy {
        Deploy {
            log,
            name,
            path,
            config,
        }
    }

    pub async fn run(&self) {
        self.log.step(format!("Running from {}", self.path).as_str());

        // Nix pack
        let env: Vec<&str> = vec![];
        let cli_plan = BuildPlan::default();
        let options = GeneratePlanOptions {
            plan: Some(cli_plan),
            config_file: None,
        };
        let mut image_version = "latest";
        let mut tags: Vec<String> = vec![];
        if let Some(version) = &self.config.version {
            image_version = version;
            tags.push(format!("{}:{}", self.name, version));
        }

        let build_options = &DockerBuilderOptions {
            name: Option::from(self.name.clone()),
            out_dir: None,
            print_dockerfile: false,
            tags,
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
        create_docker_image(&self.path, env, &options, build_options)
            .await
            .expect("\n Ruku was unable to create docker image");

        self.log
            .step(format!("Image created successfully with tag {}:{}", self.name, image_version).as_str());
    }
}
