/*
    Appellation: Pzzld <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:

        To-do:
            Application:
                Channels:
                    - Create a response matrix for the application channels
                    -
*/
pub use self::{context::*, settings::*, states::*};

pub mod api;
pub mod cli;
pub mod data;
pub mod proxy;

pub(crate) mod context;
pub(crate) mod settings;
pub(crate) mod states;

use acme::prelude::{AppSpec, AsyncSpawnable};
use acme::TokioChannelPackMPSC;
use scsys::prelude::{AsyncResult, Contextual, Locked};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> AsyncResult {
    // Initialize, then spawn the application
    Application::default().spawn().await?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Application {
    pub ctx: Context,
    pub state: Locked<State>,
}

impl Application {
    pub fn new(ctx: Context, state: Locked<State>) -> Self {
        Self { ctx, state }
    }
    // Initializes a pack of channels with a buffer of three
    pub fn state_channels(&self) -> TokioChannelPackMPSC<Locked<State>> {
        tokio::sync::mpsc::channel(3)
    }
    // Update the application state
    pub async fn update_state(&mut self, state: State) -> AsyncResult<&Self> {
        self.state = Arc::new(Mutex::new(state));
        self.state_channels().0.send(self.state.clone()).await?;
        Ok(self)
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new(Default::default(), States::default().into())
    }
}

#[async_trait::async_trait]
impl AsyncSpawnable for Application {
    async fn spawn(&mut self) -> AsyncResult<&Self> {
        self.update_state(States::Setup.into()).await?;
        self.setup()?;
        self.update_state(States::Idle.into()).await?;
        let cli = cli::new();
        tracing::info!("Success: Commands parsed, processing requests...");
        cli.handler(self.ctx.clone()).await?;
        Ok(self)
    }
}

impl From<Settings> for Application {
    fn from(cnf: Settings) -> Self {
        Self::new(Context::new(cnf), States::default().into())
    }
}

impl AppSpec<Settings> for Application {
    type Ctx = Context;

    type State = State;

    fn init() -> Self {
        Self::from(Settings::default())
    }

    fn context(&self) -> Self::Ctx {
        self.ctx.clone()
    }

    fn name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn settings(&self) -> Settings {
        self.ctx.cnf.clone()
    }

    fn setup(&mut self) -> AsyncResult<&Self> {
        // Initialize the logger
        self.settings().logger.setup(None);
        tracing_subscriber::fmt::init();

        self.state
            .lock()
            .unwrap()
            .events
            .push("Application Initialized".to_string());
        Ok(self)
    }

    fn state(&self) -> &scsys::Locked<Self::State> {
        &self.state
    }

    fn slug(&self) -> String {
        self.name().to_ascii_lowercase()
    }
}

impl Contextual for Application {
    type Cnf = Settings;

    type Ctx = Context;

    fn context(&self) -> &Self::Ctx {
        &self.ctx
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::json!({
                "name": self.name()
            })
        )
    }
}
