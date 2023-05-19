/*
    Appellation: routes <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
pub use self::{index::*, settings::*};

mod index;
mod settings;

pub mod auth;
pub mod dash;
pub mod map;
