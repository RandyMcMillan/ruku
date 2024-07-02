use nixpacks::create_docker_image;
use nixpacks::nixpacks::builder::docker::DockerBuilderOptions;
use nixpacks::nixpacks::plan::{generator::GeneratePlanOptions, BuildPlan};

use crate::container::Container;
use crate::logger::Logger;
use crate::misc::get_image_name_with_version;
use crate::model::Config;

pub struct Deploy<'a> {
    log: &'a Logger,
    name: &'a str,
    path: &'a str,
    config: &'a Config,
    container: &'a Container<'a>,
}

impl<'a> Deploy<'a> {
    pub fn new(
        log: &'a Logger,
        name: &'a str,
        path: &'a str,
        config: &'a Config,
        container: &'a Container<'a>,
    ) -> Deploy<'a> {
        Deploy {
            log,
            name,
            path,
            config,
            container,
        }
    }

    pub async fn run(&self) {
        self.log.step(&format!("Running from {}", self.path));

        // Nix pack
        let env: Vec<&str> = vec![];
        let cli_plan = BuildPlan::default();
        let options = GeneratePlanOptions {
            plan: Some(cli_plan),
            config_file: None,
        };

        let image_name_with_version = get_image_name_with_version(self.name, &self.config.version);

        let build_options = DockerBuilderOptions {
            name: Some(self.name.to_string()),
            out_dir: None,
            print_dockerfile: false,
            tags: vec![image_name_with_version.clone()],
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

        create_docker_image(self.path, env, &options, &build_options)
            .await
            .expect("\n Ruku was unable to create docker image");

        self.log.step(&format!(
            "Image created successfully with tag {}",
            image_name_with_version
        ));

        self.container.run().await;
    }
}
