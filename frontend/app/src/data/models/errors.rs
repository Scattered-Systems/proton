/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]

pub struct ErrorInfo {
    pub message: String,
}

impl ErrorInfo {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
