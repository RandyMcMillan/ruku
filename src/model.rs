use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub name: Option<String>,
    pub version: Option<String>,
}
