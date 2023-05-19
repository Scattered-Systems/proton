/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Hashable,
    PartialEq,
    Serialize,
    SmartDefault,
)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Error,
    #[default]
    Idle,
    Invalid,
    Running,
    Startup,
    Terminated,
}

impl State {
    pub fn idle() -> Self {
        Self::Idle
    }
    pub fn set(&mut self, val: Self) {
        *self = val;
    }
}

impl From<State> for u8 {
    fn from(val: State) -> u8 {
        val as u8
    }
}
