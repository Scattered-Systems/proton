/*
    Appellation: interface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{api::Api, contexts::Context, sessions::Session, states::States};
use scsys::{agents::Stateful, BoxResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Application<T: Stateful> {
    pub ctx: Context,
    pub session: Session,
    pub state: States<T>,
}

impl<T: Stateful> Application<T> {
    pub fn new(ctx: Context) -> Self {
        let session = Session::default();
        let state = Default::default();
        Self {
            ctx,
            session,
            state,
        }
    }
    pub async fn graceful_shutdown(&self) {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to terminate the runtime...");
        tracing::info!("Terminating the application and connected services...");
    }
    ///
    pub fn with_tracing(&self) -> &Self {
        let mut logger = self.ctx.settings.clone().tracing.unwrap_or_default();
        logger.setup(None);
        tracing_subscriber::fmt::init();
        tracing::info!("Successfully initiated the tracing protocol...");
        self
    }
    ///
    pub fn set_state(&mut self, state: States<T>) -> &Self {
        self.state = state;
        self
    }
    ///
    pub fn setup_backend(&self) -> Api {
        Api::new(self.ctx.settings.server.address(), self.ctx.clone())
    }
    ///
    pub async fn spawn(&self) -> BoxResult<&Self> {
        self.setup_backend().run().await?;

        Ok(self)
    }
}

impl<T: Stateful> std::fmt::Display for Application<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
