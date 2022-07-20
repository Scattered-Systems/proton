/*
    Appellation: app <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{TEXT_BOX_WIDTH, VERTICAL_WIDGET_SPACING};
pub use application::*;
use druid::WidgetExt;

mod application;
pub mod components;
pub mod faces;

#[derive(Clone, Debug, druid::Data, druid::Lens)]
pub struct AppStore {
    name: String,
}

impl AppStore {
    fn constructor(name: String) -> Result<Self, scsys::BoxError> {
        Ok(Self { name })
    }

    pub fn empty() -> Self {
        Self::constructor("".to_string()).ok().unwrap()
    }

    pub fn new(name: String) -> Self {
        Self::constructor(name).ok().unwrap()
    }

    pub fn canvas() -> druid::widget::Flex<Self> {
        let label = druid::widget::Label::new(|data: &Self, _env: &druid::Env| {
            format!("Hello {}!", data.name)
        });

        let input_name = druid::widget::TextBox::new()
            .with_placeholder("Who are we greeting?")
            .fix_width(TEXT_BOX_WIDTH)
            .lens(Self::name);

        let layout = druid::widget::Flex::column()
            .with_child(label)
            .with_spacer(VERTICAL_WIDGET_SPACING)
            .with_child(input_name);

        layout
    }

    pub fn display() -> impl druid::Widget<Self> {
        druid::widget::Align::centered(Self::canvas())
    }
}
