/*
   Appellation: users <models>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Users {
    pub password: String,
    pub username: String,
}
