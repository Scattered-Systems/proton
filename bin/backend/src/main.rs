/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[doc(inline)]
pub use self::{actors::*, components::*, core::*, data::*};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;

#[tokio::main]
async fn main() -> scsys::BoxResult {
    println!("{}", scsys::prelude::Timestamp::now());

    let app = Application::<contexts::Context>::default();
    println!("{}", &app);
    app.with_tracing().spawn().await?;

    Ok(())
}
