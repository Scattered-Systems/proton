/*
    Appellation: settings <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::ApplicationState;
use druid::{
    widget::{Flex, Label, TextBox},
    WidgetExt,
};
use scsys::BoxError;

#[derive(Clone, Debug)]
pub struct SettingsPage;

impl SettingsPage {
    pub fn account_settings() -> Flex<ApplicationState> {
        Flex::column()
            .with_flex_child(Label::new("Account Settings").center(), 1.0)
            .with_flex_child(
                Flex::row()
                    .with_flex_child(Label::new("Name"), 1.0)
                    .with_flex_child(TextBox::new().lens(ApplicationState::query), 1.0),
                1.0,
            )
    }
    pub fn application_settings() -> Flex<ApplicationState> {
        Flex::row().with_flex_child(Label::new("Application Settings").center().expand(), 1.0)
    }
    pub fn devices_settings() -> Flex<ApplicationState> {
        Flex::row().with_flex_child(Label::new("Device Settings").center().expand(), 1.0)
    }
    pub fn component() -> Flex<ApplicationState> {
        Flex::column()
            .with_flex_child(Self::account_settings(), 1.0)
            .with_flex_child(Self::application_settings(), 1.0)
            .with_flex_child(Self::devices_settings(), 1.0)
    }
    pub fn constructor() -> Result<Flex<ApplicationState>, BoxError> {
        Ok(Self::component())
    }
}
