/*
    Appellation: proton-sdk <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[cfg(feature = "backend")]
pub use proton_backend as backend;
#[cfg(feature = "core")]
pub use proton_core as core;
#[cfg(feature = "runtime")]
pub use proton_runtime as rt;

pub mod prelude {
    #[cfg(feature = "core")]
    pub use super::core::*;
    #[cfg(feature = "runtime")]
    pub use super::rt::*;
    pub use super::*;
}
