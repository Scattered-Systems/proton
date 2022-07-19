/*
    Appellation: application <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{AppStore, TEXT_BOX_WIDTH, VERTICAL_WIDGET_SPACING, WINDOW_TITLE};
use druid::{self, WidgetExt};

#[derive(Clone, Debug)]
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
    /// Create the main window
    fn constructor() -> impl druid::Widget<AppStore> {
        let label = druid::widget::Label::new(|data: &AppStore, _env: &druid::Env| {
            format!("Hello {}!", data.name)
        });

        let input_name = druid::widget::TextBox::new()
            .with_placeholder("Who are we greeting?")
            .fix_width(TEXT_BOX_WIDTH)
            .lens(AppStore::name);

        let layout = druid::widget::Flex::column()
            .with_child(label)
            .with_spacer(VERTICAL_WIDGET_SPACING)
            .with_child(input_name);

        druid::widget::Align::centered(layout)
    }
    pub fn new() -> Self {
        Self {
            dimensions: (1200.0f64, 800.0f64),
            state: AppStore::new(),
            title: WINDOW_TITLE,
        }
    }
    pub fn run(self) {
        let main_window = druid::WindowDesc::new(Self::constructor)
            .title(self.title)
            .window_size(self.dimensions);
        druid::AppLauncher::with_window(main_window)
            .launch(self.state.clone())
            .expect("Failed to launch application");
    }
}
