/*
    Appellation: application <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{AppStore, WINDOW_TITLE};
use druid::{self, WidgetExt};

#[derive(Clone, Debug)]
/// Designed to store the different face states
pub enum Pages {
    Default(AppStore),
}

#[derive(Clone, Debug)]
pub struct Proton {
    pub dimensions: (f64, f64),
    pub state: AppStore,
    pub title: druid::LocalizedString<AppStore>,
}

impl Proton {
    pub fn application(&mut self) {
        let main_window = druid::WindowDesc::new(AppStore::display)
            .title(self.title.clone())
            .window_size(self.dimensions.clone());
        druid::AppLauncher::with_window(main_window)
            .launch(self.state.clone())
            .expect("Failed to launch application");
    }
    pub fn new() -> Self {
        Self {
            dimensions: (1200.0f64, 800.0f64),
            state: AppStore::empty(),
            title: WINDOW_TITLE,
        }
    }
}
