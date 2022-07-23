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
    pub fn component() -> Flex<ApplicationState>
        where
            Self: Sized,
    {
        Flex::row()
            .with_flex_child(Label::new("Sidebar").center(), 0.75)
            .with_flex_child(Label::new("Display").center(), 3.0)
            .with_flex_child(Label::new("Feed").center(), 0.75)
    }
    pub fn constructor() -> Result<Flex<ApplicationState>, BoxError>
        where
            Self: Sized,
    {
        Ok(Flex::column().with_flex_child(Self::component(), 1.0))
    }
}
