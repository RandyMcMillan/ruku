use std::path::PathBuf;

pub struct ServerConfig {
    pub ruku_root: PathBuf,
    pub ruku_binary: PathBuf,
    pub data_root: PathBuf,
    pub git_root: PathBuf,
    pub apps_root: PathBuf,
}

impl ServerConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let home_dir = home::home_dir().ok_or("Could not determine home directory")?;
        let ruku_root = home_dir.join(".ruku");

        Ok(ServerConfig {
            ruku_root: home_dir.join(".ruku"),
            ruku_binary: PathBuf::from("/usr/bin/ruku"),
            data_root: ruku_root.join("data"),
            git_root: ruku_root.join("repos"),
            apps_root: home_dir.join("apps"),
        })
    }
}
