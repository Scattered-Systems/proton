/*
    Appellation: app <binary>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::application::Backend;

pub mod api;
mod application;

#[tokio::main]
async fn main() -> scsys::core::BoxResult {
    let app = Backend::new();
    app.with_logging()
        .run()
        .await
        .expect("Application startup failed...");
    Ok(())
}
