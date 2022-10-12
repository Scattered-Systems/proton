/*
    Appellation: proton <wasm>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
// Fix for now: https://github.com/rustwasm/wasm-bindgen/issues/2774
#![allow(clippy::unused_unit)]
pub use self::{actors::*, components::*, core::*, data::*};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;

use wasm_bindgen::prelude::*;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsError> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<interface::App>();
    Ok(())
}
