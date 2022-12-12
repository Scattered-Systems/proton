/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::types::*;

pub(crate) mod types {
    ///
    pub type Bundle<T = String> = std::collections::HashMap<T, Vec<Vec<T>>>;
}
