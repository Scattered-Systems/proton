/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{Logger, Settings};
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Context {
    pub cnf: Settings,
    pub workdir: PathBuf,
}

impl Context {
    pub fn new(cnf: Option<Settings>, workdir: Option<PathBuf>) -> Self {
        Self {
            cnf: cnf.unwrap_or_default(),
            workdir: workdir.unwrap_or_else(crate::project_root),
        }
    }
    pub fn address(&self) -> std::net::SocketAddr {
        self.cnf.server.address()
    }
    pub fn logger(&self) -> &Logger {
        &self.cnf.logger
    }
    pub fn name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
    pub fn settings(&self) -> &Settings {
        &self.cnf
    }
    pub fn set_settings(&mut self, cnf: Settings) -> &Self {
        self.cnf = cnf;
        self
    }
    pub fn workdir(&self) -> &PathBuf {
        &self.workdir
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let a = Context::default();
        let b = a.clone();
        assert_eq!(a, b)
    }
}
