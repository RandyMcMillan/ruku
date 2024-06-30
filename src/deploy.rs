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

        let config = envy::from_env::<Config>().unwrap_or_else(|e| {
            eprintln!("\n Ruku was unable to resolve the PORT environment variable");
            std::process::exit(1);
        });
    }
}