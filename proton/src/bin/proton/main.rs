/*
    Appellation: proton <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    description:
        Proton is a unique runtime environment capable of engaging a myriad of providers

*/
pub use self::{application::*, settings::*, states::*};

pub(crate) mod application;
pub(crate) mod settings;
pub(crate) mod states;

pub mod api;
pub mod cli;


#[tokio::main]
async fn main() -> scsys::prelude::BoxResult {
    Application::<String>::default().run().await?;

    Ok(())
}
