/*
    Appellation: pzzld-eth-app <wasm>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::utils::*;

extern crate cfg_if;
extern crate wasm_bindgen;

pub mod api;

pub(crate) mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello,{}!", name));
}


pub fn coinbase_api() {

}
