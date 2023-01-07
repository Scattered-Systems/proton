/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Args;
use proton_sdk::wallets::Mnemonic;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

async fn create_new_wallet() -> Mnemonic {
    let mut mnemonic = Mnemonic::new(None, None);
    mnemonic.generate(None).await.ok().unwrap();

    println!("{:?}", mnemonic);
    mnemonic.clone()
}

#[derive(Args, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Wallets {
    #[clap(long = "addr", short, value_parser)]
    address: Option<String>,
    #[clap(long, short, value_parser)]
    passphrase: Option<String>,

    #[arg(action = clap::ArgAction::SetTrue, long)]
    new: bool,
}

impl Wallets {
    pub fn new(address: Option<String>, passphrase: Option<String>) -> Self {
        let mut new = true;
        if address.is_some() && passphrase.is_some() {
            new = false;
        }
        Self {
            address,
            passphrase,
            new,
        }
    }
    pub async fn handler(&self) -> AsyncResult<&Self> {
        tracing::debug!("System processing...");
        if self.address.is_none() {}
        if self.passphrase.is_none() && self.new {
            create_new_wallet().await;
        }
        Ok(self)
    }
}
