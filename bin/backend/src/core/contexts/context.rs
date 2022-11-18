/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::Settings;
use scsys::prelude::{messages::Message, Stateful, Timestamp};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context<T: Display = Value> {
    pub message: Message<T>,
    pub settings: Settings,
    pub timestamp: i64,
}

impl<T: Display> Context<T> {
    pub fn new(message: Message<T>, settings: Settings) -> Self {
        let timestamp = Timestamp::default().into();
        Self {
            message,
            settings,
            timestamp,
        }
    }
}

impl<T: Clone + Default + Display + Serialize> Stateful for Context<T> {
    type Data = T;

    fn message(&self) -> &Message<Self::Data> {
        &self.message
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl<T: Display + Serialize> Display for Context<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}