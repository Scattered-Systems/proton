/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::platform::contexts::Context;

use scsys::prelude::{Configurable, Loggable, Logger, Stateful};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Runtime<Cnf: Configurable, S: Stateful> {
    pub cnf: Cnf,
    pub ctx: Context<Cnf>,
    pub state: S
}

impl<Cnf: Configurable, S: Stateful> Runtime<Cnf, S> {
    pub fn new(cnf: Cnf, ctx: Context<Cnf>, state: S) -> Self {
        Self { cnf, ctx, state }
    }
}

impl<Cnf: Configurable + Loggable, S: Stateful> Runtime<Cnf, S> {
    pub fn setup_logger(&self) -> &Self {
        Logger::new(self.cnf.level().clone()).setup(None);
        tracing_subscriber::fmt::init();
        self
    }
}
