/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::specs::*;

pub mod api;
pub mod ops;

pub(crate) mod specs {
    use gloo::worker::{Worker};

    pub trait Service: Worker {

        fn name(&self) -> &String;
        fn slug(&self) -> &String {
            &self.name().clone().to_lowercase()
        }
    }

}