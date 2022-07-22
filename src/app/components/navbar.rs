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

pub trait ComponentSpec<App: druid::Data = ApplicationState, Cnt = Context> {
    fn component(&mut self) -> Result<Flex<App>, BoxError>;
    fn content(&mut self) -> Result<Flex<App>, BoxError> {
        Ok(Flex::row())
    }
}

#[derive(Clone, Debug)]
pub struct Navbar {
    pub controller: Context,
}

impl Navbar {
    pub fn brand(&mut self) -> Flex<ApplicationState> {
        Flex::row().with_flex_child(Label::new(&*self.controller.name), 1.0)
    }

    pub fn content(&mut self) -> Flex<ApplicationState> {
        let mut content = Flex::row();
        for i in 0..6 {
            content.add_flex_child(
                Button::new(format!("{}", self.controller.pages.clone()[i]))
                    .on_click(move |_event, data: &mut u32, _env| {
                        *data = i.try_into().ok().unwrap();
                    })
                    .lens(ApplicationState::view),
                1.0,
            );
        }
        content
    }

    pub fn component(&mut self) -> Flex<ApplicationState> {
        let mut navbar = Flex::row();
        navbar.add_flex_child(self.brand(), 0.75);
        navbar.add_flex_child(self.content(), 3.0);
        navbar
    }

    fn constructor(controller: Context) -> Result<Self, BoxError> {
        Ok(Self { controller })
    }

    pub fn new(controller: Context) -> Self {
        match Self::constructor(controller) {
            Ok(v) => v,
            Err(e) => panic!("Component Error: {}", e),
        }
    }
}
