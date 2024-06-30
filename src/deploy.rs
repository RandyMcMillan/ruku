use std::env;
use std::path::{Path, PathBuf};

use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    port: u16,
    name: Option<String>
}

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

        // Check if a .env file exists in the current path
        let dotenv_path = Path::new(".env");
        if dotenv_path.exists() {
            dotenv().expect(".env file not found");
        }

        for (n,v) in env::vars() {
            println!("{}: {}", n, v);
        }
    }
}