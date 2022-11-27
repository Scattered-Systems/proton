/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use super::states::State;
use crate::{api::Api, cli::*, Settings};
use proton::{platform::contexts::Context, rt::Runtime};

use scsys::prelude::{BoxResult, Configurable};
use serde::{Deserialize, Serialize};
use std::fmt::Display;


#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Application<T: Clone + Default + Display + Serialize> {
    pub cnf: Settings,
    pub ctx: Context<Settings>,
    pub state: State<T>,
}

impl<T: Clone + Default + Display + Serialize> Application<T> {
    pub fn new(cnf: Settings) -> Self {
        let ctx = Context::new(cnf.clone());
        Self {
            cnf,
            ctx,
            state: Default::default(),
        }
    }
    pub fn set_state(&mut self, state: State<T>) -> &Self {
        self.state = state;
        self
    }
    pub async fn spawn_api(&self) -> BoxResult {
        let api = Api::new(self.ctx.clone());
        api.run(Some(self.cnf.server.clone())).await?;
        Ok(())
    }
    /// Initializes the platform runtime
    pub async fn runtime(&self) -> BoxResult<Runtime<Settings, State<T>>> {
        let ctx = Context::new(self.cnf.clone());
        let rt = Runtime::new(self.cnf.clone(), ctx, self.state.clone());
        rt.setup_logger();
        Ok(rt)
    }
    pub async fn run(&self) -> BoxResult<&Self> {
        let _rt = self.runtime().await?;
        tracing::info!("Success: Application initialized; awaiting commands...");
        // self.spawn_api().await?;
        let cli = CLIContext::default();
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
