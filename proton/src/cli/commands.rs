/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::args::{Services, System, Wallets};
use clap::Subcommand;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, Subcommand)]
pub enum Commands {
    Account {
        #[clap(long, short, value_parser)]
        address: String,
    },
    Services(Services),
    System(System),
    Wallet(Wallets),
}

impl Commands {
    pub async fn handler(&self, ctx: crate::Context) -> AsyncResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Account { address } => {
                println!("{:?}", address);
            }
            Self::Services(services) => {
                services.handler().await?;
            }
            Self::System(system) => {
                system.handler(ctx).await?;
            }
            Self::Wallet(wallets) => {
                wallets.handler().await?;
            }
        };
        Ok(self)
    }
}
