/*
    Appellation: proton-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Proton SDK (core)
//! 
//! This crate contains the core functionality of the Proton SDK. It is
//! intended to be used by other crates in the Proton SDK ecosystem, but
//! can be used independently.
//! 
//! ## Features

pub use self::primitives::*;

pub mod auth;
pub mod errors;
pub mod utils;

mod primitives;

pub mod prelude {
    pub use super::*;

    pub use super::auth::*;
    pub use super::errors::*;
    pub use super::utils::*;
}