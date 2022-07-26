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
    fn constructor(ens: String) -> Self {
        Self { ens }
    }
    pub fn default() -> Self {
        Self::constructor("".to_string())
    }
    pub fn new(ens: String) -> Self {
        Self::constructor(ens)
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
    fn header(controller: Context) -> Flex<Self> {
        Flex::row().with_flex_child(Navbar::new(controller).component(), 1f64)
    }
    fn body() -> druid::widget::ViewSwitcher<Self, u32> {
        Views::constructor()
    }
    fn footer() -> Flex<Self> {
        Flex::row().with_flex_child(druid::widget::Label::new("Footer").center().expand(), 1f64)
    }
    pub fn canvas() -> Flex<ApplicationState> {
        let controller = Context::default();
        Flex::column()
            .with_flex_child(Self::header(controller.clone()), 0.75)
            .with_flex_child(Self::body(), 3f64)
            .with_flex_child(Self::footer(), 0.75)
    }
    pub fn display() -> impl druid::Widget<Self> {
        druid::widget::Align::centered(Self::canvas())
    }
    pub fn new() -> Self {
        Self {
            account: AccountState::default(),
            message: String::from(""),
            query: String::from(""),
            view: 0u32,
        }
    }
}
