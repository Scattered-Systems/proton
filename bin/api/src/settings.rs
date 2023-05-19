/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use config::{Config, Environment};
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Type alias for [config::File]
type ConfigFile<Src, Fmt> = config::File<Src, Fmt>;
/// Type alias for a collection of [crate::ConfigFile]
type ConfigFileVec = Vec<ConfigFile<config::FileSourceFile, config::FileFormat>>;

/// A generic function wrapper extending glob::glob
pub fn collect_files_as<T>(f: &dyn Fn(PathBuf) -> T, pat: &str) -> anyhow::Result<Vec<T>> {
    let mut files = Vec::<T>::new();
    for r in glob::glob(pat)? {
        files.push(f(r?))
    }
    Ok(files)
}
/// Gather configuration files following the specified pattern and collect them into a vector
pub fn collect_config_files(pattern: &str, required: bool) -> ConfigFileVec {
    let f = |p: std::path::PathBuf| ConfigFile::from(p).required(required);
    collect_files_as(&f, pattern).expect("Failed to find any similar files...")
}

/// [package_name] is a simple functional wrapper for [env("CARGO_PKG_NAME")]
pub fn package_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}
/// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
pub fn project_root() -> std::path::PathBuf {
    std::path::Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
/// Attempts to collect configuration files, following the given pattern, into a ConfigFileVec
pub fn try_collect_config_files(pattern: &str, required: bool) -> anyhow::Result<ConfigFileVec> {
    let f = |p: std::path::PathBuf| ConfigFile::from(p).required(required);
    collect_files_as(&f, pattern)
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Settings {
    pub logger: Logger,
    pub mode: String,
    pub server: Server,
}

impl Settings {
    pub fn new(mode: Option<String>) -> Self {
        Self {
            logger: Default::default(),
            mode: mode.unwrap_or_else(|| String::from("production")),
            server: Default::default(),
        }
    }
    pub fn build() -> Result<Self, config::ConfigError> {
        let mut builder = Config::builder()
            .set_default("mode", "production")?
            .set_default("logger.level", "info")?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?;

        if let Ok(log) = std::env::var("RUST_LOG") {
            builder = builder.set_override("logger.level", log)?;
        };
        if let Ok(port) = std::env::var("SERVER_PORT") {
            builder = builder.set_override("server.port", port)?;
        };
        // Add in related environment variables
        builder = builder.add_source(
            Environment::default()
                .separator("__")
                .prefix(package_name().to_ascii_uppercase().as_str()),
        );
        // Try gathering valid configuration files...
        if let Ok(files) = try_collect_config_files("**/*.config.*", false) {
            builder = builder.add_source(files);
        }
        builder.build()?.try_deserialize()
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}
impl Default for Settings {
    fn default() -> Self {
        let d = Self::new(None);
        Self::build().unwrap_or(d)
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            level: tracing::Level::INFO.to_string(),
        }
    }
    pub fn set_level(mut self, level: impl ToString) {
        self.level = level.to_string();
    }
    pub fn setup_env(mut self, level: Option<&str>) -> Self {
        let key = level.unwrap_or("RUST_LOG");
        if let Some(v) = std::env::var_os(key) {
            self.level = v.into_string().expect("Failed to convert into string...");
        } else {
            std::env::set_var(key, self.level.clone());
        }
        self
    }
    pub fn init_tracing(self) {
        tracing_subscriber::fmt::init();
        tracing::debug!("Success: tracing layer initialized...");
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl From<tracing::Level> for Logger {
    fn from(level: tracing::Level) -> Self {
        Self {
            level: level.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
    pub fn address(&self) -> std::net::SocketAddr {
        format!("{}:{}", self.host, self.port).parse().unwrap()
    }
    pub fn host(&self) -> std::net::IpAddr {
        self.address().ip()
    }
    pub fn port(&self) -> u16 {
        self.address().port()
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new("0.0.0.0".to_string(), 8080)
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
