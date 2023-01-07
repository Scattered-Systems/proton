/*
   Appellation: routes <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/

pub mod auth;
pub mod index;
pub mod wasm;

pub fn api(ctx: crate::Context) -> axum::Router {
    axum::Router::new()
        .merge(index::router())
        .nest("/auth", auth::router(ctx))
}
