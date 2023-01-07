/*
    Appellation: proton-xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::utils::*;

pub(crate) mod utils;

pub mod cli;
pub mod workspace;

///
pub type Bundle<T = &str> = std::collections::HashMap<T, Vec<Vec<T>>>;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    cli::CommandLineInterface::default().handler()?;
    // cli()?;

    Ok(())
}

use clap::{command, Arg, ArgAction, Command, arg};

fn cli() -> anyhow::Result<()> {
    
    let matches = command!() // requires `cargo` feature
        .arg(arg!(-r --release "Optionally run application in release").action(ArgAction::SetTrue))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();
    


    println!("wilds: {:?}", matches);
    Ok(())
}