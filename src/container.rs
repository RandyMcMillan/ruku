use bollard::Docker;

pub struct Container {
    docker: Docker,
}

impl Container {
    pub fn new(docker: Docker) -> Container {
        Container {
            docker,
        }
    }
}