/*
   Appellation: proxy <module> [Apache-2.0]
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use acme::prelude::{Host, Port};
use axum::{
    body::{self, Body},
    response::{IntoResponse, Response},
};
use http::{Method, Request, StatusCode};
use hyper::upgrade::Upgraded;
use scsys::prelude::{Addressable, AsyncResult, IOResult};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tower::{make::Shared, ServiceExt};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Proxy {
    pub address: SocketAddr,
}

impl Proxy {
    pub fn new(host: Host, port: Port) -> Self {
        let address = SocketAddr::from((host, port));
        Self { address }
    }
    pub async fn serve(&self) -> AsyncResult {
        Ok(())
    }
}

impl Addressable for Proxy {
    type Addr = SocketAddr;

    fn address(self) -> Self::Addr {
        self.address.clone()
    }
}

pub async fn proxy(req: Request<Body>) -> AsyncResult<Response> {
    tracing::trace!(?req);

    if let Some(addr) = req.uri().authority().map(|auth| auth.to_string()) {
        // Spawn a new task for upgrading the request
        tokio::task::spawn(async move {
            match hyper::upgrade::on(req).await {
                Err(e) => {
                    // Raise a warning with the tracer
                    tracing::warn!("Upgrade Error: {}", e)
                }
                Ok(v) => {
                    if let Err(e) = tunnel(v, addr).await {
                        // Raise a warning with the tracer
                        tracing::warn!("server io error: {}", e);
                    };
                }
            }
        });
        // Create a new response
        let res = Response::new(body::boxed(body::Empty::new()));
        Ok(res)
    } else {
        // Raise a warning with the tracer
        tracing::warn!("CONNECT host is not socket addr: {:?}", req.uri());
        // Create a new response
        let res = (StatusCode::BAD_REQUEST, "CONNECT must be a socket address").into_response();
        Ok(res)
    }
}

pub async fn tunnel(mut upgraded: Upgraded, addr: String) -> IOResult {
    let mut server = TcpStream::connect(addr).await?;

    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    tracing::debug!(
        "client wrote {} bytes and received {} bytes",
        from_client,
        from_server
    );

    Ok(())
}

pub async fn spawn(router: axum::Router) -> AsyncResult {
    let service = tower::service_fn(move |req: Request<Body>| {
        let router_svc = router.clone();
        async move {
            if req.method() == Method::CONNECT {
                proxy(req).await
            } else {
                router_svc.oneshot(req).await.map_err(|err| match err {})
            }
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(Shared::new(service))
        .await
        .unwrap();
    Ok(())
}
