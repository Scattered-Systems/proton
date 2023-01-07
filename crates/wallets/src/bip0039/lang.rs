/*
    Appellation: lang <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::fnl_remove;
use serde::{Deserialize, Serialize};
use std::convert::From;
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
pub enum Language {
    #[default]
    English = 0,
    French = 1,
}

impl From<&Self> for Language {
    fn from(data: &Self) -> Self {
        *data
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            fnl_remove(&serde_json::to_string(&self).unwrap()).to_ascii_lowercase()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lang_default() {
        let a = Language::default();
        assert_eq!(a.clone(), Language::English);
        assert_eq!(a.to_string(), "english".to_string())
    }
}
