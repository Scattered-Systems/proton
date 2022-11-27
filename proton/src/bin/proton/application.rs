/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use super::states::State;
use crate::{api::Api, cli::CommandLineInterface, Context, Settings};
use scsys::prelude::{BoxResult, Configurable, Loggable};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Runtime<Cnf: Configurable, T: Default + Display> {
    pub cnf: Cnf,
    pub ctx: scsys::prelude::Context<Cnf>,
    pub state: State<T>
}

impl<Cnf: Configurable, T: Default + Display> Runtime<Cnf, T> {
    pub fn new(cnf: Cnf, ctx: scsys::prelude::Context<Cnf>, state: State<T>) -> Self {
        Self { cnf, ctx, state }
    }
}

impl<Cnf: Configurable + Loggable, T: Default + Display> Runtime<Cnf, T> {
    pub fn setup_logger(&self) -> &Self {
        scsys::prelude::Logger::new(self.cnf.level().clone()).setup(None);
        tracing_subscriber::fmt::init();
        self
    }
}


#[derive(Clone, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Application<T: Clone + Default + Display> {
    pub cnf: Settings,
    pub ctx: Context,
    pub state: State<T>,
}

impl<T: Clone + Default + Display> Application<T> {
    pub fn new(cnf: Settings) -> Self {
        let ctx = Context::new(cnf.clone());
        Self {
            cnf,
            ctx,
            state: Default::default(),
        }
    }
    pub fn setup_logger(&self) -> &Self {
        self.clone().cnf.logger.unwrap_or_default().setup(None);
        tracing_subscriber::fmt::init();

        self
    }
    pub fn set_state(&mut self, state: State<T>) -> &Self {
        self.state = state;
        self
    }
    pub async fn spawn_api(&self) -> BoxResult {
        let api = Api::new(self.ctx.clone());
        api.run().await?;
        Ok(())
    }
    pub fn cli(&self) -> CommandLineInterface {
        CommandLineInterface::default()
    }
    /// Implements the runtime, which loops between given events dependant* upon their origin
    pub async fn runtime(&self) -> BoxResult<&Self> {

        Ok(self)
    }
    pub async fn run(&self) -> BoxResult<&Self> {
        self.setup_logger();
        tracing::info!("Success: Application initialized; awaiting commands...");
        // self.spawn_api().await?;
        let cli = CommandLineInterface::default();
        tracing::info!("Success: Commands parsed, processing requests...");
        cli.handler().await;
        Ok(self)
    }
}

impl<T: Clone + Default + Display + Serialize> std::fmt::Debug for Application<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}


impl<T: Clone + Default + Display + Serialize> std::fmt::Display for Application<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
