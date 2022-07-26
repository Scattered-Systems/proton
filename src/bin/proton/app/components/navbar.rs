/*
    Appellation: navbar <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{ApplicationState, Context};
use druid::{
    widget::{Button, Flex, Label},
    WidgetExt,
};
use scsys::BoxError;

#[derive(Clone, Debug)]
pub struct Navbar {
    pub context: Context,
}

impl Navbar {
    fn constructor(context: Context) -> Self {
        Self { context }
    }
    pub fn left_gutter(&mut self) -> Flex<ApplicationState> {
        Flex::row().with_flex_child(Label::new(&*self.context.name), 1.0)
    }
    pub fn content(&mut self) -> Flex<ApplicationState> {
        let mut content = Flex::row();
        for i in 0..6 {
            content.add_flex_child(
                Button::new(format!("{}", self.context.pages.clone()[i]))
                    .on_click(move |_event, data: &mut u32, _env| {
                        *data = i.try_into().ok().unwrap();
                    })
                    .lens(ApplicationState::view),
                1.0,
            );
        }
        content
    }
    pub fn right_gutter(&mut self) -> Flex<ApplicationState> {
        let mut gutter: Flex<ApplicationState> = Flex::row();
        gutter.add_flex_child(Label::new("Settings"), 1f64);
        gutter
    }
    pub fn component(&mut self) -> Flex<ApplicationState> {
        Flex::row()
            .with_flex_child(self.left_gutter(), 0.5)
            .with_flex_spacer(0.5)
            .with_flex_child(self.content().center(), 3.0)
            .with_flex_spacer(0.5)
            .with_flex_child(self.right_gutter(), 0.5)
    }
    pub fn new(context: Context) -> Self {
        Self::constructor(context)
    }
}
