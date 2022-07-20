/*
   Appellation: Proton <binary>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Proton is designed to be your gateway into the next generation of internet-based experiences
       natively supporting a myriad of alternative blockchain networks for you to explore. Uniquely
       each user will be required to possess an ENS address as our software automatically injects
       a set of sub applications enabling users to retain global access to their data and devices.
*/

#![windows_subsystem = "windows"]
#[doc(inline)]
pub use crate::{app::*, core::*, cyber::*, data::*};

mod app;
mod core;
mod cyber;
mod data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Proton");

    Proton::init().application();

    Ok(())
}
