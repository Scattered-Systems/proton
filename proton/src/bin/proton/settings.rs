/*
   Appellation: settings
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use scsys::prelude::{config::{Config, Environment}, try_collect_config_files, ConfigResult, Configurable, Logger, Server};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub logger: Option<Logger>,
    pub server: Server,
}

impl Settings {
    pub fn new(logger: Option<Logger>, server: Server) -> Self {
        Self { logger, server }
    }
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .add_source(Environment::default().separator("__"))
            .set_default("logger.level", Some("info"))?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 9000)?;

        match try_collect_config_files("**/Proton.toml", false) {
            Err(_) => {},
            Ok(v) => {builder = builder.add_source(v);}
        }
        
        match std::env::var("RUST_LOG") {
            Err(_) => {},
            Ok(v) => {builder = builder.set_override("logger.level", Some(v))?;}
        };

        match std::env::var("SERVER_PORT") {
            Err(_) => {},
            Ok(v) => {builder = builder.set_override("server.port", v)?;}
        };

        builder.build()?.try_deserialize()
    }
}

impl Configurable for Settings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl scsys::prelude::Loggable for Settings {
    fn level(&self) -> String {
        self.logger.clone().unwrap_or_default().level
    }
}

impl Default for Settings {
    fn default() -> Self {
        match Self::build() {
            Ok(v) => v,
            Err(_) => Self::new(Some(Default::default()), Server::new("127.0.0.1".to_string(), 9000)),
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

