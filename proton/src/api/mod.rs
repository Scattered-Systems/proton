/*
   Appellation: api <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::interface::*;

pub mod routes;

pub fn new() -> Api {
    Api::default()
}

pub fn from_context(ctx: crate::Context) -> Api {
    Api::new(ctx.clone())
}

pub(crate) mod interface {
    use crate::{api::routes, Context};
    use acme::net::servers::{Server, ServerSpec};
    use acme::prelude::net::WebBackend;
    use axum::Router;
    use http::header::{HeaderName, AUTHORIZATION};
    use scsys::prelude::{AsyncResult, Hash, Hashable};
    use serde::{Deserialize, Serialize};
    use tower_http::{
        compression::CompressionLayer,
        propagate_header::PropagateHeaderLayer,
        sensitive_headers::SetSensitiveHeadersLayer,
        trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    };

    #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct Api {
        pub ctx: Context,
        pub server: Server,
    }

    impl Api {
        pub fn new(ctx: Context) -> Self {
            let server = Server::from(ctx.cnf.server.pieces());
            Self { ctx, server }
        }
        /// Quickstart the server with the outlined client
        pub async fn serve(&self) -> AsyncResult {
            self.server().serve(self.client().await).await
        }
    }

    #[async_trait::async_trait]
    impl WebBackend for Api {
        type Ctx = Context;

        type Server = Server;

        async fn client(&self) -> axum::Router {
            Router::new()
                .nest("/app", routes::wasm::router())
                .merge(routes::api(self.ctx.clone()))
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
                .layer(axum::Extension(self.ctx.clone()))
        }

        fn context(&self) -> Self::Ctx {
            self.ctx.clone()
        }

        fn server(&self) -> Self::Server {
            self.server.clone()
        }
    }

    impl std::fmt::Display for Api {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", serde_json::to_string(&self).ok().unwrap())
        }
    }
}
