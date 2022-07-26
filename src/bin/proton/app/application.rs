/*
    Appellation: application <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{ApplicationState, Context, Navbar, Views};
use druid::{widget::Flex, WidgetExt};
use scsys::BoxError;

#[derive(Clone, Debug)]
pub struct App {
    pub controller: Context,
    pub state: ApplicationState,
}

impl App {
    fn constructor(controller: Context, state: ApplicationState) -> Self {
        Self { controller, state }
    }
    pub fn application(self) {
        let win = create_window(self.controller.win.shape);

        druid::AppLauncher::with_window(win)
            .launch(self.state.clone())
            .expect("Application Error: Application failed to launch");
    }
    pub fn new() -> Self {
        Self::constructor(Context::default(), ApplicationState::init())
    }
}

pub fn create_window(shape: (f64, f64)) -> druid::WindowDesc<ApplicationState> {
    let display = ApplicationState::display;
    druid::WindowDesc::new(display)
        .title(druid::LocalizedString::new("Proton"))
        .window_size(shape)
}

pub fn application_launcher(window: druid::WindowDesc<ApplicationState>) {
    druid::AppLauncher::with_window(window)
        .launch(ApplicationState::init())
        .expect("Application Error: Application failed to launch")
}
