/*
    Appellation: application <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{DefaultStore, WINDOW_TITLE};
use druid::{self, WidgetExt, WindowDesc};

#[derive(Clone, Debug)]
/// Designed to store the different face states
pub enum Pages {
    Default(DefaultStore),
}

#[derive(Clone, Debug)]
pub struct Proton {
    pub dimensions: (f64, f64),
    pub state: DefaultStore,
    pub title: druid::LocalizedString<DefaultStore>,
}

impl Proton {
    pub fn application(&mut self) {
        druid::AppLauncher::with_window(self.display())
            .launch(self.state.clone())
            .expect("Failed to launch application");
    }
    pub fn display(&mut self) -> WindowDesc<DefaultStore> {
        druid::WindowDesc::new(DefaultStore::display)
            .title(self.title.clone())
            .window_size(self.dimensions.clone())
    }
    pub fn new() -> Self {
        Self {
            dimensions: (1200.0f64, 800.0f64),
            state: DefaultStore::empty(),
            title: WINDOW_TITLE,
        }
    }
}
