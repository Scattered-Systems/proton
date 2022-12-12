/*
    Appellation: core <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{application::*, primitives::*, requests::*, utils::*};

pub mod contexts;
pub mod errors;
pub mod hooks;
pub mod routes;

pub(crate) mod application;
pub(crate) mod primitives;
pub(crate) mod requests;

/// Quickstart the application
pub fn start() -> proton_sdk::prelude::BoxResult {
    yew::Renderer::<App>::new().render();

    Ok(())
}

pub(crate) mod utils {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn add_one(data: usize) -> usize {
        data + 1
    }
}
