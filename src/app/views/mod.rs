/*
    Appellation: mod <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use account::*;
pub use community::*;
pub use control_panel::*;
pub use creator::*;
pub use dashboard::*;
pub use discover::*;

mod account;
mod community;
mod control_panel;
mod creator;
mod dashboard;
mod discover;

use crate::ApplicationState;
use druid::{
    widget::{Button, Flex, Label, Split, TextBox, ViewSwitcher},
    Env, WidgetExt,
};
use scsys::BoxError;

pub trait PageSpec<As: druid::Data = ApplicationState> {
    fn component() -> Flex<As> where Self: Sized;
    fn constructor() -> Result<Flex<As>, scsys::BoxError> where Self: Sized;
}


#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Views;

impl Views {
    pub fn constructor() -> ViewSwitcher<ApplicationState, u32> {
        ViewSwitcher::new(
            |data: &ApplicationState, _env| data.view,
            |selector, _data, _env| match selector {
                0 => Box::new(Dashboard::constructor().ok().unwrap()),
                1 => Box::new(AccountView::constructor().ok().unwrap()),
                2 => Box::new(CommunityCenter::constructor().ok().unwrap()),
                3 => Box::new(Self::discover()),
                4 => Box::new(Self::creation_hub()),
                _ => Box::new(Self::control_panel()),
            },
        )
    }


    /// Combining a block explorer, global marketplace, and search engine into a single portal
    pub fn discover() -> Flex<ApplicationState> {
        Flex::column()
            .with_flex_child(Label::new("Here is a label").center(), 1.0)
            .with_flex_child(
                Button::new("Button").on_click(|_event, _data, _env| {
                    println!("Complex button clicked!");
                }),
                1.0,
            )
            .with_flex_child(TextBox::new().lens(ApplicationState::query), 1.0)
            .with_flex_child(
                Label::new(|data: &String, _env: &Env| format!("Value entered: {}", data))
                    .lens(ApplicationState::query),
                1.0,
            )
    }

    /// Equipped with all the tools needed to create content on the decentralized web
    pub fn creation_hub() -> Flex<ApplicationState> {
        Flex::column()
            .with_flex_child(Label::new("Here is a label").center(), 1.0)
            .with_flex_child(
                Button::new("Button").on_click(|_event, _data, _env| {
                    println!("Complex button clicked!");
                }),
                1.0,
            )
            .with_flex_child(TextBox::new().lens(ApplicationState::query), 1.0)
            .with_flex_child(
                Label::new(|data: &String, _env: &Env| format!("Value entered: {}", data))
                    .lens(ApplicationState::query),
                1.0,
            )
    }

    /// A mashup of account, application, and device settings
    pub fn control_panel() -> Flex<ApplicationState> {
        Flex::column()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("Account Settings").center(), 1.0)
                    .with_flex_child(
                        Flex::row()
                            .with_flex_child(Label::new("Name"), 1.0)
                            .with_flex_child(TextBox::new().lens(ApplicationState::query), 1.0),
                        1.0,
                    ),
                1.0,
            )
            .with_flex_child(
                Flex::row()
                    .with_flex_child(Label::new("Application Settings").center().expand(), 1.0),
                1.0,
            )
            .with_flex_child(
                Flex::row().with_flex_child(Label::new("Device Settings").center().expand(), 1.0),
                1.0,
            )
    }
}
