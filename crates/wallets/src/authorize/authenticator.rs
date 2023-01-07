/*
    Appellation: authenticator <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        With the recent advent of a multi-device authentication protocol (FIDO's PassKey) developers
        are enabled to string together devices, uniting them under a single credential which is
        typically stored under the user's primary service provider; i.e. Apple, Google, etc.

*/
use super::IAuthenticator;
use scsys::prelude::Dictionary;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AuthState {
    Authorized,
    Owner,
    UnAuthorized,
}

/// Implement a secure authenticator
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Authenticator {
    pub authorizations: Dictionary,
    pub endpoint: String,
}

impl Authenticator {
    pub fn new(authorizations: Dictionary, endpoint: String) -> Self {
        Self {
            authorizations,
            endpoint,
        }
    }
}

impl IAuthenticator<String, Vec<String>> for Authenticator {}

impl Default for Authenticator {
    fn default() -> Self {
        Self::new(Dictionary::new(), String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_authenticator() {
        let actual = Authenticator::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
