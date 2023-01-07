/*
    Appellation: portals <middleware>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        (def) Portal middleware implments capabilities for integrating content and utilities
                from various applications onto a single screen creating a single, composite application
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Portal {

}

impl Portal {
    pub fn new() -> Self {
        Self {}
    }
}

