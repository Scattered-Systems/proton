/*
    Appellation: wrappers <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{auth::*, errors::*, tags::*};

pub(crate) mod auth;
pub(crate) mod errors;
pub(crate) mod tags;
