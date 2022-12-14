/*
    Appellation: proton-xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{primitives::*, utils::*};

pub(crate) mod utils;

pub mod cli;
pub mod wasm;
pub mod workspace;

pub(crate) mod primitives;

use proton_sdk::prelude::BoxResult;

fn main() -> BoxResult {
    tracing_subscriber::fmt::init();
    cli::CommandLineInterface::default().handler()?;
    Ok(())
}

pub trait BuildPipeline {
    fn init() -> Self;
    fn handle(&self) -> BoxResult<&Self>;
    fn run(&mut self) -> BoxResult;
    fn stage(&self) -> String;
}
