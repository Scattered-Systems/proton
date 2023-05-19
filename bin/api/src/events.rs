/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault as Default;
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Hashable,
    PartialEq,
    Serialize,
)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum AppEvent {
    #[default]
    Power(PowerEvent),
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Hashable,
    PartialEq,
    Serialize,
)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum PowerEvent {
    Shutdown,
    #[default]
    Startup,
}
