/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{context::*, events::*, primitives::*, settings::*, states::*};

mod context;
mod events;
mod primitives;
mod settings;
mod states;

pub mod api;

use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    {
        let app = Application::default().init();
        app.spawn(tokio::runtime::Handle::current()).await??;
    }
    Ok(())
}

#[derive(Debug)]
pub struct Application {
    pub ctx: Arc<Context>,
    event: mpsc::Receiver<AppEvent>,
    pub state: Locked<State>,
}

impl Application {
    pub fn new(ctx: Arc<Context>, event: mpsc::Receiver<AppEvent>) -> Self {
        Self {
            ctx: ctx.clone(),
            event,
            state: Arc::new(Mutex::new(State::default())),
        }
    }
    pub fn api(&self) -> api::Api {
        api::Api::new(self.ctx.clone())
    }
    /// Handle events from the event loop
    pub async fn handle_event(&mut self, event: AppEvent) -> anyhow::Result<()> {
        match event {
            AppEvent::Power(power) => match power {
                PowerEvent::Shutdown => {
                    tokio::signal::ctrl_c().await?;
                    tracing::info!("Message Recieved: Shutting down...");
                    self.state.lock().unwrap().set(State::Terminated);
                }
                PowerEvent::Startup => {
                    tracing::info!("Message Recieved: System initializing...");
                    self.state.lock().unwrap().set(State::Startup);
                }
            },
        }

        Ok(())
    }
    /// Initialize the application
    pub fn init(self) -> Self {
        let logger = self.ctx.settings().logger.clone();
        logger.setup_env(None).init_tracing();
        self
    }
    /// Process commands
    pub async fn handle_command(&mut self, _cmd: impl ToString) -> anyhow::Result<()> {
        Ok(())
    }
    /// Run the application
    pub async fn run(mut self) -> anyhow::Result<()> {
        Ok(loop {
            tokio::select! {
                Some(event) = self.event.recv() => {
                    tracing::info!("Event Recieved: {:?}", event);
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Message Recieved: Shutting down...");
                    break;
                }
            }
        })
    }
    /// Spawn the application
    pub fn spawn(
        self,
        handle: tokio::runtime::Handle,
    ) -> tokio::task::JoinHandle<anyhow::Result<()>> {
        let api = api::Api::new(self.ctx.clone());

        handle.spawn(async move {
            api.serve().await?;
            self.run().await
        })
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new(Arc::new(Context::default()), mpsc::channel(1).1)
    }
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.ctx.as_ref()).unwrap())
    }
}
