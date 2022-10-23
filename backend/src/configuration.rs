use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub client_port: u16,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    Config::builder()
        .add_source(File::with_name("configuration"))
        .build()?
        .try_deserialize()
}
