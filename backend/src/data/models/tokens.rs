/*
    Appellation: tokens <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use scsys::prelude::StringGenerator;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TokenModel {
    pub access_token: String,
    pub token_type: String,
    pub username: Option<String>,
}

impl TokenModel {
    pub fn new(access_token: String, token_type: String, username: Option<String>) -> Self {
        Self {
            access_token,
            token_type,
            username,
        }
    }
    pub fn generate_access_token(&mut self) -> Self {
        self.access_token = StringGenerator::new(32).data;
        self.clone()
    }
}

impl Default for TokenModel {
    fn default() -> Self {
        Self::new(String::new(), String::new(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::TokenModel;

    #[test]
    fn test_token_model() {
        let a = TokenModel::default().generate_access_token();
        let b = TokenModel::default().generate_access_token();
        assert_ne!(a, b)
    }
}
