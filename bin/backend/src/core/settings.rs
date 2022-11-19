/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use config::{Config, Environment};
use scsys::{
    collect_config_files,
    ConfigResult,
    prelude::{
        Logger, Server,
        
    },
};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub mode: Option<String>,
    pub name: Option<String>,
    pub server: Server,
    pub tracing: Option<Logger>,
}

impl Settings {
    pub fn build() -> ConfigResult<Self> {
        let builder = Config::builder()
            .add_source(
               collect_config_files("**/Backend.toml", false)
            )
            .add_source(Environment::default().prefix("APP").separator("__"));

        builder.build()?.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        match Self::build() {
            Ok(v) => v,
            Err(e) => {
                println!("Please check your configuration file: {}", e);
                Self {
                    mode: Some("prod".to_string()),
                    name: Some("backend".to_string()),
                    server: Default::default(),
                    tracing: Some(Default::default()),
                }
            }
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
