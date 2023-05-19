/*
    Appellation: key <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::SecpKeypair;
use rand::rngs::OsRng;
use secp256k1::Secp256k1;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WalletKey {
    pub public: String,
    pub secret: String,
    pub timestamp: i64,
}

impl WalletKey {
    pub fn new(public: String, secret: String) -> Self {
        Self {
            public,
            secret,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    pub fn generate_keypair(&mut self) -> &Self {
        let (sk, pk) = Secp256k1::new().generate_keypair(&mut OsRng);
        self.public = pk.to_string();
        self.secret = sk.display_secret().to_string();
        self
    }
    // TODO: Find a better method of converting a SecretKey into a String
    pub fn from_keypair(keypair: SecpKeypair) -> Self {
        Self::new(keypair.1.to_string(), format!("{:?}", keypair.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_keys() {
        let actual = WalletKey::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
