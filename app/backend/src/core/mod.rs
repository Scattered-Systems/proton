/*
   Appellation: core <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{
    context::Context, errors::*, primitives::*, settings::Settings, state::*, utils::*,
};

mod context;
pub(crate) mod errors;
mod primitives;
mod settings;
mod state;

mod utils {}
