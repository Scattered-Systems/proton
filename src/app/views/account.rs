/*
    Appellation: account <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::ApplicationState;
use druid::{WidgetExt, widget::{Flex, Label}};
use scsys::BoxError;

#[derive(Clone, Debug)]
pub struct AccountView;

impl AccountView {
    pub fn component() -> Flex<ApplicationState> where Self: Sized {
        Flex::row().with_flex_child(Label::new("ENS").center(), 1.0)
    }

    pub fn constructor() -> Result<Flex<ApplicationState>, BoxError> where Self: Sized {
        Ok(Flex::column().with_flex_child(
            Self::component(),
            1.0,
        ))
    }
}