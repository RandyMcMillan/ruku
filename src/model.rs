use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Config {
    pub port: Option<u16>,
    pub version: Option<String>,
}
