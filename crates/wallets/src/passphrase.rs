/*
    Appellation: passphrase <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Passphrase(String);

impl Passphrase {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn generate(len: usize) -> Self {
        Self::new(crate::generate_random_string(len))
    }
    pub fn passphrase(&self) -> &String {
        &self.0
    }
}

impl Default for Passphrase {
    fn default() -> Self {
        Self::generate(12)
    }
}

impl std::fmt::Display for Passphrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: ToString> From<&T> for Passphrase {
    fn from(data: &T) -> Self {
        Self::new(data.to_string())
    }
}

impl From<usize> for Passphrase {
    fn from(data: usize) -> Self {
        Self::generate(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passphrase() {
        let actual = Passphrase::default();
        let expected = Passphrase::default();
        assert_ne!(actual, expected)
    }
}
