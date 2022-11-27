/*
   Appellation: client <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::Context;
use axum::Router;
use http::header::{HeaderName, AUTHORIZATION};
use scsys::prelude::Contextual;
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

#[derive(Clone, Debug)]
pub struct ClientRouter(pub Router);

impl ClientRouter {
    pub fn new() -> Self {
        Self(Router::new())
    }
    pub fn layers(&mut self, ctx: Context) -> &Self {
        self.0 = self.router().clone()
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
            .layer(axum::Extension(ctx));
        self
    }
    pub fn router(&self) -> &Router {
        &self.0
    }
}