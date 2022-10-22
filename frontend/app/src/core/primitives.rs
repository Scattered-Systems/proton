/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, statics::*, types::*};

pub(crate) mod constants {
    pub const API_ROOT: &str = "http://localhost:8000/api";
    pub const TOKEN_KEY: &str = "yew.token";
}

pub(crate) mod statics {
    use super::TOKEN_KEY;
    use gloo::storage::{LocalStorage, Storage};
    use lazy_static::lazy_static;
    use parking_lot::RwLock;

    lazy_static! {
        /// Jwt token read from local storage.
        pub static ref TOKEN: RwLock<Option<String>> = {
            if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
                RwLock::new(Some(token))
            } else {
                RwLock::new(None)
            }
        };
    }
}

pub(crate) mod types {
    use std::collections::HashMap;

    pub type DeleteWrapper = HashMap<(), ()>;
}
