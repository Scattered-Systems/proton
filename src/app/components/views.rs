/*
    Appellation: views <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::ApplicationState;
use druid::{
    widget::{Button, Flex, Label, Split, TextBox, ViewSwitcher},
    Env, WidgetExt,
};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Views;

impl Views {
    pub fn constructor() -> ViewSwitcher<ApplicationState, u32> {
        ViewSwitcher::new(
            |data: &ApplicationState, _env| data.view,
            |selector, _data, _env| match selector {
                0 => Box::new(Self::dashboard()),
                1 => Box::new(Self::account()),
                2 => Box::new(Self::communication_center()),
                3 => Box::new(Self::discover()),
                4 => Box::new(Self::creation_hub()),
                _ => Box::new(Self::control_panel()),
            },
        )
    }

    /// Describes the default view for our application
    pub fn dashboard() -> Flex<ApplicationState> {
        Flex::column().with_flex_child(
            Flex::row()
                .with_flex_child(Label::new("Sidebar").center(), 0.75)
                .with_flex_child(Label::new("Display").center(), 3.0)
                .with_flex_child(Label::new("Feed").center(), 0.75),
            1.0,
        )
    }

    /// Describes the account view (index: 1) for our application
    pub fn account() -> Flex<ApplicationState> {
        Flex::column().with_flex_child(
            Flex::row().with_flex_child(Label::new("ENS").center(), 1.0),
            1.0,
        )
    }

    /// Describes
    pub fn communication_center() -> Split<ApplicationState> {
        let stream: Vec<&str> = vec!["AppFeed", "Person"];
        println!("{:#?}", stream.clone());
        let feed = Flex::column().with_flex_child(Label::new("Feed").center().expand(), 1.0);
        let editor = Flex::column()
            .with_flex_child(Label::new("Message").center().expand(), 0.25)
            .with_flex_child(Label::new("History").center().expand(), 1.0)
            .with_flex_child(TextBox::new().lens(ApplicationState::message), 0.25);
        Split::columns(feed, editor).draggable(true)
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
