/*
    Appellation: application <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use druid::{
    LocalizedString,
    WindowDesc,
    WidgetExt,
};
use scsys::BoxError;

#[derive(Clone, Debug)]
pub struct App {
    pub shape: (f64, f64),
    pub state: crate::HomeSpace,
}

impl App {
    pub fn application(&mut self) {
        /// Initialize a WindowDesc<T>
        let display = WindowDesc::new(crate::HomeSpace::display)
            .title(LocalizedString::new("Proton"))
            .window_size(self.shape.clone());

        /// Call the built-in launcher with display (d)
        druid::AppLauncher::with_window(display)
            .launch(self.state.clone())
            .expect("Failed to launch application");
    }
    fn constructor(shape: (f64, f64), state: crate::HomeSpace) -> Result<Self, BoxError> {
        Ok(Self { shape, state })
    }
    pub fn new(shape: (f64, f64), state: crate::HomeSpace) -> Self {
        match Self::constructor(shape, state) {
            Ok(v) => v,
            Err(e) => panic!("Application Error: {}", e),
        }
    }
    pub fn init() -> Self {
        Self::new((1200f64, 800f64), crate::HomeSpace::init())
    }
}

