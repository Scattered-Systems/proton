/*
    Appellation: interface <api>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{api::routes, Settings};
use axum::{Router, Server};
use http::header::{HeaderName, AUTHORIZATION};
use proton::platform::contexts::Context;
use scsys::prelude::{BoxResult, Configurable, Contextual};
use serde::{Deserialize, Serialize};
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

pub enum ApiState {
    Connected,
    Disconnected,
    Detached,
    Idle
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Api {
    pub ctx: Context<Settings>,
}

impl Api {
    pub fn new(ctx: Context<Settings>) -> Self {
        Self { ctx }
    }
    /// Creates a new client instance for the server to run which is responsible for collecting implemented routes and desired attributes
    pub async fn client(&self) -> Router {
        let mut router = Router::new();
        // Merge availible routers into the base client
        router = router
            .merge(routes::Homepage::default().router());
        // Add depth to the service with layers
        router = router
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                AUTHORIZATION,
            )))
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(HeaderName::from_static(
                "x-request-id",
            )))
            .layer(axum::Extension(self.ctx.clone()));
        router
    }
    /// Implements a graceful shutdown when users press CTRL + C
    pub async fn shutdown(&self) {
        tokio::signal::ctrl_c()
            .await
            .expect("Expect shutdown signal handler");
        tracing::info!("Terminating the application...");
    }
    /// Quickly run the api
    pub async fn run(&self, server: Option<scsys::prelude::Server>) -> BoxResult {
        let address = server.unwrap_or_default().address();
        let client = self.client().await;
        let server = Server::bind(&address)
            .serve(client.into_make_service())
            .with_graceful_shutdown(self.shutdown())
            .await?;
        Ok(server)
    }
}

impl Contextual for Api {
    type Cnf = Settings;

    type Ctx = Context<Settings>;

    fn context(&self) -> &Self::Ctx {
        &self.ctx
    }
}

impl Configurable for Api {
    type Settings = crate::Settings;

    fn settings(&self) -> &Self::Settings {
        &self.ctx.settings
    }
}

impl std::fmt::Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",  serde_json::to_string(&self).unwrap())
    }
}
