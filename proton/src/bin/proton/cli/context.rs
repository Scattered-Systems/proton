/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::Commands;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, clap::ValueEnum)]
pub enum ApplicationArgs {
    #[default]
    Update,
}

#[derive(Clone, Debug, Deserialize, Hash, Parser, PartialEq, Serialize)]
#[clap(about, author, version)]
#[clap(long_about = "")]
pub struct CLIContext {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[arg(long, short)]
    pub update: Option<isize>,
}

impl CLIContext {
    pub async fn handler(&self) -> &Self {
        match self.command.clone() {
            None => {}
            Some(v) => {
                v.handler().await;
            }
        }
        self
    }
}

impl Default for CLIContext {
    fn default() -> Self {
        Self::parse()
    }
}
