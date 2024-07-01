use nixpacks::create_docker_image;
use nixpacks::nixpacks::builder::docker::DockerBuilderOptions;
use nixpacks::nixpacks::plan::{generator::GeneratePlanOptions, BuildPlan};

pub struct Deploy {
    name: String,
    path: String,
    port: u16,
}

impl Deploy {
    pub fn new(name: String, path: String, port: u16) -> Deploy {
        Deploy { name, path, port }
    }

    pub async fn run(&self) {
        println!("Deploying from {}", self.path);

        // Nix pack
        let env: Vec<&str> = vec![];
        let cli_plan = BuildPlan::default();
        let options = GeneratePlanOptions {
            plan: Some(cli_plan),
            config_file: None,
        };
        let build_options = &DockerBuilderOptions {
            name: Option::from(self.name.clone()),
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
        create_docker_image(&self.path, env, &options, build_options)
            .await
            .expect("\n Ruku was unable to create docker image");
    }
}
