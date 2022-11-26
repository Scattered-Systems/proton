/*
    Appellation: core <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{primitives::*, utils::*};

pub(crate) mod primitives {


}

pub(crate) mod utils {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn add_one(data: usize) -> usize {
        data + 1
    }

    #[wasm_bindgen]
    pub fn timestamp() -> String {
        chrono::Utc::now().to_rfc3339()
    }
}