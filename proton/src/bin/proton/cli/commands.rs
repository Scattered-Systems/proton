/*
    Appellation: commands <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::api::Api;
use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, Eq, PartialEq, Serialize, Subcommand)]
pub enum Commands {
    Account {
        #[clap(long, short, value_parser)]
        address: String,
    },
    Services {
        #[arg(long, short)]
        update: Option<isize>,
    },
    System {
        #[arg(action = clap::ArgAction::SetTrue, long)]
        up: bool,
    },
}

impl Commands {
    pub async fn handler(&self) -> scsys::AsyncResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Account { address } => {
                println!("{:?}", &address);
            }
            Self::Services { update } => {
                println!("{:?}", &update);
            }
            Self::System { up } => {
                if up.clone() {
                    tracing::info!("Spawning the api...");
                    // tokio::spawn(async move {app.spawn_api();});
                    let api = Api::default();
                    match api.run(Some(crate::Settings::default().server)).await {
                        Err(e) => panic!("{}", e),
                        Ok(v) => v,
                    };
                }
            }
        };
        Ok(self)
    }
}
