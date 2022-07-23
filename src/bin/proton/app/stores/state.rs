/*
    Appellation: state <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{Context, Navbar, Views};
use druid::{widget::Flex, WidgetExt};
use scsys::BoxError;

#[derive(Clone, Debug, druid::Data, druid::Lens)]
pub struct AccountState {
    pub ens: String,
}

impl AccountState {
    fn constructor(ens: String) -> Result<Self, BoxError> {
        Ok(Self { ens })
    }
    pub fn init() -> Self {
        Self::new("".to_string())
    }
    pub fn new(ens: String) -> Self {
        Self::constructor(ens).ok().unwrap()
    }
}

#[derive(Clone, Debug, druid::Data, druid::Lens)]
pub struct ApplicationState {
    pub account: AccountState,
    pub message: String,
    pub query: String,
    pub view: u32,
}

impl ApplicationState {
    pub fn canvas() -> Result<Flex<ApplicationState>, BoxError> {
        let controller = Context::default();
        Ok(Flex::column()
            .with_flex_child(Navbar::new(controller.clone()).component(), 0.75)
            .with_flex_child(Views::constructor(), 3f64)
            .with_flex_child(
                Flex::row()
                    .with_flex_child(druid::widget::Label::new("Footer").center().expand(), 1f64),
                0.75,
            ))
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
            account: AccountState::init(),
            message,
            query,
            view,
        })
    }
}
