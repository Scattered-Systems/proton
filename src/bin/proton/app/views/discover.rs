/*
    Appellation: discover <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::ApplicationState;
use druid::{
    widget::{Button, Flex, Label, TextBox},
    Env, WidgetExt,
};
use scsys::BoxError;

#[derive(Clone, Debug)]
pub struct DiscoverPage;

impl DiscoverPage {
    fn header() -> Flex<ApplicationState> {
        Flex::row().with_flex_child(Label::new("Discover").center().expand(), 1.0)
    }
    fn body() -> Flex<ApplicationState> {
        Flex::column()
            .with_flex_child(
                Button::new("Button")
                    .on_click(|_event, _data, _env| {
                        println!("Complex button clicked!");
                    })
                    .center(),
                1.0,
            )
            .with_flex_child(
                TextBox::new()
                    .lens(ApplicationState::query)
                    .center()
                    .expand(),
                1.0,
            )
            .with_flex_child(
                Label::new(|data: &String, _env: &Env| format!("Value entered: {}", data))
                    .lens(ApplicationState::query),
                1.0,
            )
    }
    fn footer() -> Flex<ApplicationState> {
        Flex::row().with_flex_child(Label::new("Discover").center(), 1.0)
    }
    pub fn component() -> Flex<ApplicationState> {
        Flex::column()
            .with_flex_child(Self::header(), 1.0)
            .with_flex_spacer(1f64)
            .with_flex_child(Self::body(), 1.0)
            .with_flex_spacer(1f64)
            .with_flex_child(Self::footer(), 1.0)
    }
    pub fn constructor() -> Result<Flex<ApplicationState>, BoxError> {
        Ok(Self::component())
    }
}
