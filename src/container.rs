use bollard::container::{CreateContainerOptions, StartContainerOptions};
use bollard::models::{HostConfig, PortBinding, PortMap};
use bollard::Docker;

use crate::logger::Logger;
use crate::misc::get_image_name_with_version;
use crate::model::Config;

use std::collections::HashMap;

pub struct Container<'a> {
    log: &'a Logger,

    name: &'a str,
    docker: &'a Docker,
    config: &'a Config,
}

impl<'a> Container<'a> {
    pub fn new(log: &'a Logger, name: &'a str, docker: &'a Docker, config: &'a Config) -> Container<'a> {
        Container {
            log,
            name,
            docker,
            config,
        }
    }

    pub async fn run(&self) {
        let image_name_with_version = get_image_name_with_version(self.name, &self.config.version);

        let create_options = CreateContainerOptions {
            name: self.name,
            platform: None,
        };

        let exposed_port = format!("{}/tcp", self.config.port);
        let mut host_config = HostConfig::default();
        let mut port_bindings = PortMap::new();
        port_bindings.insert(
            exposed_port.clone(),
            Some(vec![PortBinding {
                host_ip: None,
                host_port: Some(self.config.port.to_string()),
            }]),
        );
        host_config.port_bindings = Some(port_bindings);

        let mut exposed_ports_map: HashMap<String, HashMap<(), ()>> = HashMap::new();
        exposed_ports_map.insert(exposed_port, HashMap::new());

        let create_container_config = bollard::container::Config {
            image: Some(image_name_with_version),
            host_config: Some(host_config),
            exposed_ports: Some(exposed_ports_map),
            ..Default::default()
        };

        // Create the container
        let container = self
            .docker
            .create_container(Some(create_options), create_container_config)
            .await
            .unwrap_or_else(|_| {
                self.log.error("Failed to create container");
                std::process::exit(1);
            });
        self.log.step(&format!("Created container with id: {}", container.id));

        // Start the container
        self.docker
            .start_container(&container.id, None::<StartContainerOptions<String>>)
            .await
            .unwrap_or_else(|_| {
                self.log.error("Failed to start container");
                std::process::exit(1);
            });
        self.log.step(&format!("Started container with id: {}", container.id));
    }
}
