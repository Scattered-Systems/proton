/*
    Appellation: proton <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Proton is a unified interface built as a personal computing platform
*/
pub use self::{actors::*, components::*, core::*, data::*};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;

