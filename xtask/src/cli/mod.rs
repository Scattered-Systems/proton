/*
    Appellation: cli <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{commands::*, interface::*};

pub(crate) mod commands;

pub fn new() -> CommandLineInterface {
    CommandLineInterface::default()
}

pub(crate) mod interface {
    use super::Commands;
    use clap::Parser;
    use proton_sdk::prelude::BoxResult;

    #[derive(Clone, Debug, Hash, Parser, PartialEq)]
    #[clap(about, author, version)]
    #[clap(long_about = None)]
    pub struct CommandLineInterface {
        #[clap(subcommand)]
        pub command: Option<Commands>,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub debug: bool,
        #[arg(action = clap::ArgAction::SetTrue, long)]
        pub desktop: bool,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub release: bool,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        pub update: bool,
    }

    impl CommandLineInterface {
        pub fn handler(&self) -> BoxResult<&Self> {
            if self.debug {
                
            }
            if let Some(cmds) = &self.command {
                cmds.handler(self.desktop.clone(), self.release.clone())?;
            }
            Ok(self)
        }
    }

    impl Default for CommandLineInterface {
        fn default() -> Self {
            Self::parse()
        }
    }
}
