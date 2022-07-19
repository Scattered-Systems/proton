/*
    Appellation: app <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use application::*;

mod application;
pub mod components;
pub mod faces;

pub trait Gui<D: druid::Data> {
    fn application(&self) -> Result<Self, scsys::BoxError>
        where
            Self: Sized;
    fn constructor(&self) -> Result<Box<dyn druid::Widget<D>>, scsys::BoxError>
        where
            Self: Sized;
    fn data(&self) -> Result<Vec<D>, scsys::BoxError>
        where
            Self: Sized;
}

#[derive(Clone, Debug, druid::Data, druid::Lens)]
pub struct AppStore {
    ensname: String,
    name: String,
}

impl AppStore {
    fn constructor(ensname: String, name: String) -> Result<Self, scsys::BoxError> {
        Ok(Self { ensname, name })
    }

    pub fn new() -> Self {
        Self::constructor("".to_string(), "".to_string())
            .ok()
            .unwrap()
    }
}
