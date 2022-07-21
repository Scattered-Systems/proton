/*
    Appellation: application <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use druid::{LocalizedString, WidgetExt, WindowDesc};

#[derive(Clone, Debug)]
pub struct App {
    pub shape: (f64, f64),
    pub state: crate::ApplicationState,
}

impl App {
    pub fn application(&mut self) {
        let display = WindowDesc::new(crate::ApplicationState::display)
            .title(LocalizedString::new("Proton"))
            .window_size(self.shape.clone());

        druid::AppLauncher::with_window(display)
            .launch(self.state.clone())
            .expect("Failed to launch application");
    }
    pub fn new(shape: (f64, f64), state: crate::ApplicationState) -> Result<Self, scsys::BoxError> {
        Ok(Self { shape, state })
    }
    pub fn init() -> Self {
        let controller = crate::Controller::default();

        Self::new(controller.window.shape, crate::ApplicationState::init())
            .ok()
            .unwrap()
    }
}
