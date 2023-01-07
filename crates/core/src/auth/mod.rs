/*
    Appellation: auth <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::misc::*;

pub mod oauth2;

pub(crate) mod misc {
    use serde::{Deserialize, Serialize};

    // The user data we'll get back from Google.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        pub email: String,
        pub name: String,
        pub picture: Option<String>,
        pub sub: String,
    }
}
