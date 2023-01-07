/*
    Appellation: auth <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub mod oauth;
pub mod siwe;

pub fn router(ctx: crate::Context) -> axum::Router {
    axum::Router::new()
        .nest("/oauth", oauth::router(ctx))
        .nest("/siwe", siwe::router())
}
