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
    // app.with_logging().run().await?;
    sample::docker_sample().await?;
    Ok(())
}

pub(crate) mod sample {
    use bollard::Docker;
    use futures::FutureExt;

    pub async fn docker_sample() -> scsys::core::BoxResult {
        let docker = Docker::connect_with_local_defaults()?;
        let version = docker.version().await.unwrap();
        println!("{:?}", version);
        Ok(())
    }
}
