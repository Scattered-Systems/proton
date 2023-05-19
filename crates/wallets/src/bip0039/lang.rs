/*
    Appellation: lang <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
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
