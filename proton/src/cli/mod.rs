/*
    Appellation: cli <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{args::Power, commands::*, context::*};

pub(crate) mod args;
pub(crate) mod commands;

pub fn new() -> Cli {
    Cli::default()
}

pub(crate) mod context {
    use super::Commands;
    use clap::Parser;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Eq, Hash, Parser, PartialEq, Serialize)]
    #[clap(about, author, version)]
    #[clap(long_about = "Welcome to Proton")]
    pub struct Cli {
        #[clap(subcommand)]
        pub command: Option<Commands>,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub debug: bool,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub update: bool,
    }

    impl Cli {
        pub fn new() -> Self {
            Self::parse()
        }
        pub async fn handler(&self) -> scsys::AsyncResult<&Self> {
            if let Some(cmd) = self.command.clone() {
                cmd.handler().await?;
            }
            Ok(self)
        }
    }

    impl Default for Cli {
        fn default() -> Self {
            Self::new()
        }
    }
}
