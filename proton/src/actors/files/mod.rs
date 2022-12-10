/*
    Appellation: files <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{interface::*, message::*};

pub(crate) mod interface;
pub(crate) mod message;

pub type Chunks = bool;
