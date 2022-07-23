/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use constants::*;
pub use types::*;

mod constants {
    pub const VIEW_LABELS: [&str; 6] = [
        "Dashboard",
        "Account",
        "Connect",
        "Discover",
        "Create",
        "Settings",
    ];
    pub const VERTICAL_WIDGET_SPACING: f64 = 20.0;
    pub const TEXT_BOX_WIDTH: f64 = 200.0;
    pub const DEFAULT_WINDOW_SHAPE: crate::WindowShape = (800f64, 600f64);

    pub mod theme {
        pub const DARK_GREY: druid::Color = druid::Color::grey8(0x3a);
        pub const DARKER_GREY: druid::Color = druid::Color::grey8(0x11);
        pub const LIGHTER_GREY: druid::Color = druid::Color::grey8(0xbb);

        #[derive(Clone, Debug)]
        pub struct Shades {
            pub gradients: Vec<druid::LinearGradient>,
            pub shades: Vec<druid::Color>,
        }

        #[derive(Clone, Debug)]
        pub struct Colorway {
            pub alternative: Shades,
            pub dark: Shades,
            pub light: Shades,
            pub primary: Shades,
            pub secondary: Shades,
        }

        #[derive(Clone, Debug)]
        pub enum ThemeMode {
            Dark,
            Light,
        }

        impl ThemeMode {
            pub fn default() -> Self {
                Self::Dark
            }
        }

        #[derive(Clone, Debug)]
        pub struct Theme {
            pub mode: ThemeMode,
        }

        impl Theme {
            pub fn constructor(mode: ThemeMode) -> Result<Self, scsys::BoxError> {
                Ok(Self { mode })
            }
            pub fn new(mode: ThemeMode) -> Self {
                match Self::constructor(mode) {
                    Ok(v) => v,
                    Err(e) => {
                        panic!("Theme Error: {}", e)
                    }
                }
            }
            pub fn default() -> Self {
                Self::new(ThemeMode::default())
            }
        }
    }
}

mod types {
    pub type WindowShape = (f64, f64);
    pub type LString<S> = druid::LocalizedString<S>;
}
