/*
    Appellation: home <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use druid::WidgetExt;

#[derive(Clone, Debug, druid::Data, druid::Lens)]
pub struct HomeSpace {
    user: String,
}

#[derive(Clone, Debug)]
pub struct HomePage {
    pub default: bool,
    pub space: HomeSpace,
}
