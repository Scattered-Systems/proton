/*
    Appellation: app <binary>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{context::Context, interface::RESTBackend, settings::Settings};

pub mod api;

pub(crate) mod context;
pub(crate) mod interface;
pub(crate) mod settings;

use scsys::BoxResult;

#[tokio::main]
async fn main() -> BoxResult {
    let app = RESTBackend::new();
    app.with_logging().run().await?;
    Ok(())
}

pub(crate) mod sample {
    use bollard::Docker;
    use futures::FutureExt;

    pub async fn docker_sample() -> scsys::BoxResult {
        let docker = Docker::connect_with_local_defaults()?;
        let version = docker.version().await.unwrap();
        println!("{:?}", version);
        Ok(())
    }
}
