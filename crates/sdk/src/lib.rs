/*
    Appellation: proton-sdk <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Proton SDK
//! 
//! 
//! 
//! ## Features
#[cfg(feature = "core")]
pub use proton_core as core;
#[cfg(feature = "wallets")]
pub use proton_wallets as wallets;

pub mod prelude {
    #[cfg(feature = "core")]
    pub use super::core::prelude::*;
    #[cfg(feature = "wallets")]
    pub use super::wallets::*;
}
