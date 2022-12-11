/*
    Appellation: proton-sdk <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[cfg(feature = "core")]
pub use proton_core as core;

pub mod prelude {
    #[cfg(feature = "core")]
    pub use super::core::*;
    pub use super::*;
}
