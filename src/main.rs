pub use crate::{app::*, core::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Proton");

    let mut proton = Proton::new();
    proton.run();
    Ok(())
}

mod app {
    use crate::{TEXT_BOX_WIDTH, VERTICAL_WIDGET_SPACING, WINDOW_TITLE};
    use druid::{self, Widget, WidgetExt};

    pub trait Gui {
        fn constructor() -> dyn Widget<Store>;
    }

    #[derive(Clone, Debug, druid::Data, druid::Lens)]
    pub struct Store {
        ensname: String,
        name: String,
    }

    #[derive(Clone, Debug)]
    pub struct Proton {
        pub initial_state: Store,
    }

    impl Proton {
        pub fn new() -> Self {
            Self {
                initial_state: Store {
                    ensname: "".to_string(),
                    name: "".to_string(),
                },
            }
        }
        /// Create the main window
        fn constructor() -> impl Widget<Store> {
            let label = druid::widget::Label::new(|data: &Store, _env: &druid::Env| {
                format!("Hello {}!", data.name)
            });

            let input_name = druid::widget::TextBox::new()
                .with_placeholder("Who are we greeting?")
                .fix_width(TEXT_BOX_WIDTH)
                .lens(Store::name);

            let layout = druid::widget::Flex::column()
                .with_child(label)
                .with_spacer(VERTICAL_WIDGET_SPACING)
                .with_child(input_name);

            druid::widget::Align::centered(layout)
        }
        pub fn run(&mut self) {
            let main_window = druid::WindowDesc::new(Self::constructor)
                .title(WINDOW_TITLE)
                .window_size((800.0, 800.0));
            druid::AppLauncher::with_window(main_window)
                .launch(self.initial_state.clone())
                .expect("Failed to launch application");
        }
    }
}

mod core {
    use druid::LocalizedString;

    pub const VERTICAL_WIDGET_SPACING: f64 = 20.0;
    pub const TEXT_BOX_WIDTH: f64 = 200.0;
    pub const WINDOW_TITLE: LocalizedString<crate::Store> = LocalizedString::new("Proton");

    pub enum WindowParams {
        Appellation { name: String },
    }

    #[derive(Clone, Debug, Hash, PartialEq)]
    pub struct Constants;
}
