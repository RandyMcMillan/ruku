use std::path::PathBuf;

pub struct Deploy {
    pub path: PathBuf
}

impl Deploy {
    pub fn new(path: PathBuf) -> Deploy {
        Deploy {
            path
        }
    }

    pub fn run(&self) {
        println!("Deploying from {}", self.path.display());
    }
}