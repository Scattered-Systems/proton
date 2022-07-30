/*
    Appellation: creator <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        All users will delight in the unique creator portal, which is designed to remove all the
        barriers to entry when it comes to creating decentralized content while further supporting
        a number of direct pipelines to centralized access points.
*/
use crate::ApplicationState;
use druid::{
    widget::{Button, Flex, Label, TextBox},
    Env, WidgetExt,
};
use scsys::BoxError;

#[derive(Clone, Debug)]
pub struct CreatorPage;

impl CreatorPage {
    pub fn component() -> Flex<ApplicationState> {
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
    pub fn constructor() -> Result<Flex<ApplicationState>, BoxError> {
        Ok(Self::component())
    }
}
