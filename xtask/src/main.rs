/*
    Appellation: proton-xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::utils::*;

pub(crate) mod utils;

pub mod cli;

///
pub type Bundle<T = &'static str> = std::collections::HashMap<T, Vec<Vec<T>>>;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    cli::CommandLineInterface::default().handler()?;
    // cli()?;

    Ok(())
}

use clap::{command, Arg, ArgAction, Command, arg, value_parser};
use std::path::PathBuf;

pub fn cli() -> anyhow::Result<()> {
    
    let matches = command!() // requires `cargo` feature
        .subcommand(
            Command::new("action")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .arg(
            arg!(-p --port <PORT> "Configure the port")
                .value_parser(value_parser!(u16))
                .default_value("8080"),
        )
        .arg(arg!(config: -c --config <CONFIG>).value_parser(value_parser!(PathBuf)).default_value("/xtask.yml"))
        .arg(Arg::new("debug").long("debug").short('d').action(ArgAction::SetTrue))
        .arg(Arg::new("release").long("release").short('r').action(ArgAction::SetTrue))
        .arg(Arg::new("verbose").long("verbose").short('v').action(ArgAction::SetTrue))
        .get_matches();
    
    let args = matches
        .get_many::<String>("cmd")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    println!("wilds: {:?}", args);
    Ok(())
}