/*
    Appellation: scope <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
use dioxus::prelude::Props;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(rename_all = "lowercase")]
pub enum AuthMode {
    #[default]
    Anonymous,
    Authenticated,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Props, Serialize)]
pub struct Subscope {
    pub auth_mode: AuthMode,
}

impl Subscope {
    pub fn new() -> Self {
        Self {
            auth_mode: AuthMode::Anonymous,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Props, Serialize)]
pub struct ApplicationScope {
    pub name: String,
    pub content: String,
}

impl ApplicationScope {
    pub fn new() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            content: env!("CARGO_PKG_DESCRIPTION").to_string(),
        }
    }
    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn with_content(mut self, content: impl ToString) -> Self {
        self.content = content.to_string();
        self
    }
}

impl Default for Subscope {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ApplicationScope {
    fn default() -> Self {
        Self::new()
    }
}
