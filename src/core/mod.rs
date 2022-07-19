/*
    Appellation: mod <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

use druid::LocalizedString;

pub const VERTICAL_WIDGET_SPACING: f64 = 20.0;
pub const TEXT_BOX_WIDTH: f64 = 200.0;
pub const WINDOW_TITLE: LocalizedString<crate::Store> = LocalizedString::new("Proton");

pub enum WindowParams {
    Appellation { name: String },
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Constants;
