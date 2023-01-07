/*
    Appellation: args <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{services::*, system::*, wallets::*};

pub(crate) mod services;
pub(crate) mod system;
pub(crate) mod wallets;
