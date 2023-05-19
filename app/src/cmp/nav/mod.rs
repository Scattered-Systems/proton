/*
    Appellation: nav <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/

pub mod navbar;

use dioxus::prelude::Props;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Props)]

pub struct Navigation {
    pub banner: String,
}
