/*
    Appellation: core <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{interface::*, primitives::*, requests::*, utils::*};

pub mod context;
pub mod errors;
pub mod hooks;
pub(crate) mod interface;
pub(crate) mod primitives;
pub(crate) mod requests;
pub mod routes;
pub(crate) mod utils;
