use crate::logger::Logger;
use bollard::Docker;

pub struct Container<'a> {
    log: &'a Logger,

    name: &'a str,
    docker: &'a Docker,
}

impl<'a> Container<'a> {
    pub fn new(log: &'a Logger, name: &'a str, docker: &'a Docker) -> Container<'a> {
        Container { log, name, docker }
    }
}
