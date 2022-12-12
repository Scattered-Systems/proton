/*
    Appellation: proton-xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use proton_sdk::prelude::BoxResult;
use proton_xtask::{dist_dir, project_root};

fn main() -> BoxResult {
    tracing_subscriber::fmt::init();
    proton_xtask::cli::CommandLineInterface::default().handler()?;
    Ok(())
}

pub trait BuildPipeline {
    fn init() -> Self;
    fn handle(&self) -> BoxResult<&Self>;
    fn run(&mut self) -> BoxResult;
    fn stage(&self) -> String;
}
