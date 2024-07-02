use crate::logger::Logger;
use bollard::Docker;

pub struct Container<'a> {
    log: &'a Logger,

    name: String,
    docker: Docker,
}

impl<'a> Container<'a> {
    pub fn new(log: &Logger, name: String, docker: Docker) -> Container {
        Container { log, name, docker }
    }
}
