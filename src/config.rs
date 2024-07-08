use std::path::PathBuf;
use crate::logger::Logger;

pub struct Config {
    pub ruku_root: PathBuf,
    pub data_root: PathBuf,
    pub git_root: PathBuf,
    pub app_root: PathBuf,
}

impl Config {
    pub fn new(log: Logger) -> Result<Self, Box<dyn std::error::Error>> {
        let home_dir = home::home_dir().ok_or("Could not determine home directory")?;
        let ruku_root = home_dir.join(".ruku");

        let data_root = ruku_root.join("data");
        let git_root = ruku_root.join("repos");
        let app_root = home_dir.join("apps");

        Ok(Config {
            ruku_root,
            data_root,
            git_root,
            app_root,
        })
    }
}
