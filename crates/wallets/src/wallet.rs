/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/
use crate::WalletKey;
use anyhow::Result;
use secp256k1::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Describes the standard wallets interface for digital currencies
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Wallet {
    pub address: String,
    pub key: WalletKey,
    pub label: String,
}

impl Wallet {
    pub fn new(address: String, key: WalletKey, label: String) -> Self {
        Self {
            address,
            key,
            label,
        }
    }
    pub fn from_file(file_path: &str) -> Result<Self> {
        let file = std::fs::OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = std::io::BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }
    pub fn public_key(&self) -> Result<PublicKey> {
        Ok(PublicKey::from_str(&self.key.public)?)
    }
    pub fn save_to_file(&self, path: &str) -> Result<Self> {
        crate::save_to_file(self.clone(), path)
    }
    pub fn secret_key(&self) -> Result<SecretKey> {
        Ok(SecretKey::from_str(&self.key.secret)?)
    }
    pub fn set_label<T: std::string::ToString>(&mut self, label: T) -> &Self {
        self.label = label.to_string();

        self
    }
    pub fn setup(&mut self, label: String) -> Self {
        let address = String::new();
        let wallet_key = WalletKey::default().generate_keypair().clone();
        Self::new(address, wallet_key, label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet() {
        let actual = Wallet::default().set_label("test".to_string()).clone();
        let expected = actual.clone();
        assert_eq!(
            &format!("{:#?}", actual.key),
            &format!("{:#?}", expected.key)
        );
        assert_eq!(
            &format!("{:#?}", actual.label),
            &format!("{:#?}", expected.label)
        )
    }
}
