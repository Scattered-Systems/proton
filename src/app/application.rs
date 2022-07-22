/*
    Appellation: application <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{ApplicationState, Controller, Navbar, Views};
use druid::{widget::Flex, WidgetExt};
use scsys::BoxError;

#[derive(Clone, Debug)]
pub struct App {
    pub controller: Controller,
    pub state: ApplicationState,
}

impl App {
    pub fn application(&mut self) {
        let display = druid::WindowDesc::new(ApplicationState::display)
            .title(druid::LocalizedString::new("Proton"))
            .window_size(self.controller.window.shape.clone());

        druid::AppLauncher::with_window(display)
            .launch(self.state.clone())
            .expect("Application Error: Application failed to launch");
    }
    pub fn new(controller: Controller, state: ApplicationState) -> Result<Self, BoxError> {
        Ok(Self { controller, state })
    }
    pub fn init() -> Self {
        match Self::new(Controller::default(), ApplicationState::init()) {
            Ok(v) => v,
            Err(e) => panic!("Application Error: {}", e),
        }
    }
}
