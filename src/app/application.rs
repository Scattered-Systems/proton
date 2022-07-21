/*
    Appellation: application <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use druid::WidgetExt;

#[derive(Clone, Debug)]
pub struct App {
    pub controller: crate::Controller,
    pub state: crate::ApplicationState,
}

impl App {
    pub fn application(&mut self) {
        let display = druid::WindowDesc::new(crate::ApplicationState::display)
            .title(druid::LocalizedString::new("Proton"))
            .window_size(self.controller.window.shape.clone());

        druid::AppLauncher::with_window(display)
            .launch(self.state.clone())
            .expect("Application Error: Application failed to launch");
    }
    pub fn new(
        controller: crate::Controller,
        state: crate::ApplicationState,
    ) -> Result<Self, scsys::BoxError> {
        Ok(Self { controller, state })
    }
    pub fn init() -> Self {
        match Self::new(
            crate::Controller::default(),
            crate::ApplicationState::init(),
        ) {
            Ok(v) => v,
            Err(e) => panic!("Application Error: {}", e),
        }
    }
}
