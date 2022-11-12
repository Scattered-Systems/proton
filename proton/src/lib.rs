/*
    Appellation: curiosity <lib>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Curiosity is a sandbox wasm environment
*/

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_one(data: usize) -> usize {
    data + 1
}


