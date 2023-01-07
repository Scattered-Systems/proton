/*
   Appellation: settings
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use scsys::prelude::config::{Config, Environment};
use scsys::Hash;
use scsys::{
    prelude::{Configurable, Hashable, Logger, Server, SerdeDisplay},
    try_collect_config_files, ConfigResult,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, SerdeDisplay, Serialize)]
pub struct Cache {
    pub name: String,
    pub uri: String
}

impl Default for Cache {
    fn default() -> Self {
        Self { name: Default::default(), uri: "redis://0.0.0.0:6379".to_string() }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, SerdeDisplay, Serialize)]
pub struct OAuth2Config {
    #[serde(rename = "client_id")]
    pub id: String,
    #[serde(rename = "client_secret")]
    pub secret: String,
    pub redirect: String,
    pub scope: Option<String>,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, SerdeDisplay, Serialize)]
pub struct Settings {
    pub auth: OAuth2Config,
    pub cache: Cache,
    pub logger: Logger,
    pub mode: String,
    pub server: Server,
}

impl Settings {
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder();
        // Set defaults
        builder = builder
            .set_default("auth.id", "")?
            .set_default("auth.secret", "")?
            .set_default("auth.redirect", "http://localhost:8080/api")?
            .set_default("auth.token", "")?
            .set_default("cache.uri", "redis://0.0.0.0:6379")?
            .set_default("mode", "production")?
            .set_default("logger.level", "info")?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?;
        // Load in the .env file
        builder = builder.add_source(Environment::default().separator("__"));
        // Load in configuration files following the *.config.* pattern
        if let Ok(v) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(v);
        }
        // Check for alternative environment variable representations
        if let Ok(v) = std::env::var("CLIENT_ID") {
            builder = builder.set_override("auth.id", v)?;
        }
        if let Ok(v) = std::env::var("REDIRECT_URL") {
            builder = builder.set_override("auth.redirct", v)?;
        }
        if let Ok(v) = std::env::var("TOKEN_URL") {
            builder = builder.set_override("auth.token", v)?;
        }
        if let Ok(v) = std::env::var("CLIENT_SECRET") {
            builder = builder.set_override("auth.secret", v)?;
        }
        if let Ok(v) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", v)?;
        }
        if let Ok(v) = std::env::var("SERVER_PORT") {
            builder = builder.set_override("server.port", v)?;
        }
        // Attempt to build, then deserialize the configuration
        builder.build()?.try_deserialize()
    }
}

impl Configurable for Settings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for OAuth2Config {
    fn default() -> Self {
        Self {
            id: String::new(),
            secret: String::new(),
            redirect: String::from("http://localhost:8080"),
            scope: None,
            token: String::new(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        if let Ok(v) = Self::build() {
            v
        } else {
            Self {
                auth: OAuth2Config::default(),
                cache: Cache::default(),
                logger: Logger::new("info".to_string()),
                mode: "development".to_string(),
                server: Server::new("0.0.0.0".to_string(), 8080),
            }
        }
    }
}
