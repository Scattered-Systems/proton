/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use constants::*;
pub use types::*;

mod constants {
    pub const VERTICAL_WIDGET_SPACING: f64 = 20.0;
    pub const TEXT_BOX_WIDTH: f64 = 200.0;

    pub mod theme {
        pub const DARK_GREY: druid::Color = druid::Color::grey8(0x3a);
        pub const DARKER_GREY: druid::Color = druid::Color::grey8(0x11);
        pub const LIGHTER_GREY: druid::Color = druid::Color::grey8(0xbb);
    }
}

mod types {
    pub type WindowShape = (f64, f64);
    pub type LString<S> = druid::LocalizedString<S>;

    pub enum WindowParams {
        Appellation { name: String },
    }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Constants;
