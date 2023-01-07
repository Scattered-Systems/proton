/*
   Appellation: context <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use super::Settings;
use scsys::prelude::{Configurable, Contextual, Hash, Hashable, SerdeDisplay};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, SerdeDisplay, Serialize)]
pub struct Context {
    pub cnf: Settings,
}

impl Context {
    pub fn new(cnf: Settings) -> Self {
        Self { cnf }
    }
}

impl Configurable for Context {
    type Settings = Settings;

    fn settings(&self) -> &Self::Settings {
        &self.cnf
    }
}

impl Contextual for Context {
    type Cnf = Settings;
    type Ctx = Self;

    fn context(&self) -> &Self::Ctx {
        self
    }
}
