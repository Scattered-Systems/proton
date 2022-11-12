/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::{
    components::{logging::Logger, networking::Server},
    prelude::{
        collect_config_files,
        config::{Config, Environment},
        ConfigResult,
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
            .add_source(collect_config_files("**/Backend.toml", true))
            .add_source(Environment::default().with_prefix("APP").separator("__"));

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



lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name("app/backend/Backend.toml")).unwrap();

        settings
    });
}
