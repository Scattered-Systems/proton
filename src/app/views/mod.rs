/*
    Appellation: mod <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use account::*;
pub use community::*;
pub use control_panel::*;
pub use creator::*;
pub use dashboard::*;
pub use discover::*;

mod account;
mod community;
mod control_panel;
mod creator;
mod dashboard;
mod discover;

use crate::ApplicationState;
use druid::{
    widget::{Flex, ViewSwitcher},
    WidgetExt,
};
use scsys::BoxError;

pub trait PageSpec<As: druid::Data = ApplicationState> {
    fn component() -> Result<Box<dyn druid::Widget<As>>, BoxError>
        where
            Self: Sized;
    fn constructor() -> Result<Box<dyn druid::Widget<As>>, BoxError>
        where
            Self: Sized;
}

#[derive(Clone, Debug)]
pub struct Views;

impl Views {
    pub fn constructor() -> ViewSwitcher<ApplicationState, u32> {
        ViewSwitcher::new(
            |data: &ApplicationState, _env| data.view,
            |selector, _data, _env| match selector {
                0 => Box::new(Dashboard::constructor().ok().unwrap()),
                1 => Box::new(AccountView::constructor().ok().unwrap()),
                2 => Box::new(CommunityCenter::constructor().ok().unwrap()),
                3 => Box::new(DiscoverPage::constructor().ok().unwrap()),
                4 => Box::new(CreatorPage::constructor().ok().unwrap()),
                _ => Box::new(ControlPanel::constructor().ok().unwrap()),
            },
        )
    }
}
