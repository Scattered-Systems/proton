/*
    Appellation: community <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::ApplicationState;
use druid::{
    widget::{Flex, Label, Split, TextBox},
    WidgetExt,
};
use scsys::BoxError;

#[derive(Clone, Debug)]
pub struct CommunityCenter;

impl CommunityCenter {
    pub fn component() -> (Flex<ApplicationState>, Flex<ApplicationState>)
        where
            Self: Sized,
    {
        let stream: Vec<&str> = vec!["AppFeed", "Person"];
        println!("{:#?}", stream.clone());
        let feed = Flex::column().with_flex_child(Label::new("Feed").center().expand(), 1.0);
        let editor = Flex::column()
            .with_flex_child(Label::new("Message").center().expand(), 0.25)
            .with_flex_child(Label::new("History").center().expand(), 1.0)
            .with_flex_child(TextBox::new().lens(ApplicationState::message), 0.25);
        (feed, editor)
    }

    pub fn constructor() -> Result<Split<ApplicationState>, BoxError>
        where
            Self: Sized,
    {
        let (feed, editor) = Self::component();
        Ok(Split::columns(feed, editor).draggable(true))
    }
}
