#![windows_subsystem = "windows"]

pub use crate::{app::*, comp::*, core::*, data::*};

mod app;
mod comp;
mod core;
mod data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Proton");

    // let mut proton = Proton::new();
    // proton.run();

    run_calc();
    Ok(())
}
