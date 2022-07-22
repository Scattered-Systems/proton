/*
    Appellation: state <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{Controller, Navbar, Views};
use druid::{widget::Flex, WidgetExt};
use scsys::BoxError;

#[derive(Clone, Debug, druid::Data, druid::Lens)]
pub struct ApplicationState {
    pub message: String,
    pub query: String,
    pub view: u32,
}

impl ApplicationState {
    pub fn canvas() -> Result<Flex<ApplicationState>, BoxError> {
        let controller = Controller::default();
        Ok(
            Flex::column()
                .with_flex_child(Navbar::new(controller.clone()).component(), 1.0)
                .with_flex_child(Views::constructor(), 1.0)
        )
    }
    pub fn display() -> impl druid::Widget<Self> {
        druid::widget::Align::centered(Self::canvas().ok().unwrap())
    }
    pub fn init() -> Self {
        Self::new(String::from(""), String::from(""), 0u32)
            .ok()
            .unwrap()
    }
    pub fn new(message: String, query: String, view: u32) -> Result<Self, BoxError> {
        Ok(Self {
            message,
            query,
            view,
        })
    }
}
