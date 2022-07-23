/*
    Appellation: dashboard <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::ApplicationState;
use druid::{
    widget::{Flex, Label},
    WidgetExt,
};
use scsys::BoxError;

#[derive(Clone, Debug)]
pub struct Dashboard;

impl Dashboard {
    fn sidebar() -> Flex<ApplicationState> {
        Flex::column().with_flex_child(Label::new("Sidebar").center().expand(), 1.0)
    }
    fn display() -> Flex<ApplicationState> {
        Flex::column().with_flex_child(Label::new("Display").center().expand(), 1.0)
    }
    fn feed() -> Flex<ApplicationState> {
        Flex::column().with_flex_child(Label::new("Feed").center().expand(), 1.0)
    }
    pub fn component() -> Flex<ApplicationState> {
        Flex::row()
            .with_flex_child(Self::sidebar(), 0.75)
            .with_flex_child(Self::display(), 3.0)
            .with_flex_child(Self::feed(), 0.75)
    }
    pub fn constructor() -> Result<Flex<ApplicationState>, BoxError> {
        Ok(Flex::column().with_flex_child(Self::component().expand(), 1.0))
    }
}
