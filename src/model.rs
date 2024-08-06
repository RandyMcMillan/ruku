use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct RukuConfig {
    pub port: u16,
    pub version: Option<String>,
}
