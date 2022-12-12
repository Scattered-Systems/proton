/*
    Appellation: proton <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

fn main() -> proton_sdk::prelude::BoxResult {
    proton_wasm::start()?;

    Ok(())
}
