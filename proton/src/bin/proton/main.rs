/*
    Appellation: proton <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    description:
        Proton is a unique runtime environment capable of engaging a myriad of providers

*/
pub use self::{settings::*, states::*};

pub(crate) mod settings;
pub(crate) mod states;

pub mod api;
pub mod cli;

use proton::{platform::contexts::Context, rt::Runtime};
use scsys::prelude::{AsyncResult, Configurable};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> AsyncResult {
    let mut app = Application::<String>::default();
    app.start().await?;

    Ok(())
}

#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Application<T: Clone + Default + Display + Serialize> {
    pub cnf: Settings,
    pub ctx: Context<Settings>,
    pub state: Arc<State<T>>,
}

impl<T: Clone + Default + Display + Serialize> Application<T> {
    pub fn new(cnf: Settings) -> Self {
        let ctx = Context::new(cnf.clone());
        Self {
            cnf,
            ctx,
            state: Arc::new(Default::default()),
        }
    }
    /// Initializes the platform runtime
    pub async fn runtime(&mut self) -> AsyncResult<Runtime<Settings, State<T>>> {
        self.set_state(&State::from(&States::Startup));
        let rt = Runtime::new(self.cnf.clone(), self.ctx.clone(), self.state.clone());
        rt.setup_logger();
        let cli = cli::new();
        tracing::info!("Success: Commands parsed, processing requests...");
        cli.handler().await?;
        Ok(rt)
    }
    /// Update the application state
    pub fn set_state(&mut self, state: &State<T>) -> &Self {
        self.state = Arc::new(state.clone());
        self
    }
    /// Quickstart the application
    pub async fn start(&mut self) -> AsyncResult<&Self> {
        self.runtime().await?;
        Ok(self)
    }
}

impl<T: Clone + Default + Display + Serialize> Configurable for Application<T> {
    type Settings = Settings;

    fn settings(&self) -> &Self::Settings {
        &self.cnf
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
