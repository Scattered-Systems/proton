/*
    Appellation: proton-xtask <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::utils::*;

pub(crate) mod utils;

pub mod wasm;
pub mod workspace;

pub type BoxError = Box<dyn std::error::Error>;
pub type BaseResult<T = ()> = Result<T, Box<dyn std::error::Error>>;
