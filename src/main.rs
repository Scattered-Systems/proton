pub use crate::{app::*, core::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Proton");

    let proton = Proton::new();
    &application.run();
    Ok(())
}

mod app {
    use crate::{TEXT_BOX_WIDTH, VERTICAL_WIDGET_SPACING, WINDOW_TITLE};
    use druid::{Widget, WidgetExt, self};


    pub struct Face {
        pub height: f64,
        pub width: f64,
    }


    #[derive(Clone, druid::Data, druid::Lens)]
    pub struct Store {
        name: String,
    }

    #[derive(Clone, Debug)]
    pub struct Proton {
        pub name: String,
    }

    impl Proton {
        pub fn new() -> Self {
            Self { name: "Proton".to_string() }
        }

        /// Create the main window
        fn constructor() -> impl Widget<Store> {
            let label = druid::widget::Label::new(
                |data: &Store, _env: &druid::Env| format!("Hello {}!", data.name)
            );

            let textbox = druid::widget::TextBox::new()
                .with_placeholder("Who are we greeting?")
                .fix_width(TEXT_BOX_WIDTH)
                .lens(Store::name);

            let layout = druid::widget::Flex::column()
                .with_child(label)
                .with_spacer(VERTICAL_WIDGET_SPACING)
                .with_child(textbox);

            druid::widget::Align::centered(layout)
        }

        pub fn run(&self) {
            let main_window = druid::WindowDesc::new(Self::constructor)
                .title(WINDOW_TITLE)
                .window_size((400.0, 400.0));

            let initial_state = Store {
                name: "World".into(),
            };

            druid::AppLauncher::with_window(main_window)
                .launch(initial_state)
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
        Appellation {
            name: String
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq)]
    pub struct Constants;
}