/*
   Appellation: api <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{interface::*, utils::*};

pub(crate) mod interface;
pub mod client;
pub mod routes;
pub mod server;

pub(crate) mod utils {
    use super::Api;
    use crate::Context;
    use scsys::prelude::BoxResult;

    pub async fn spawn_api_with_ctx(ctx: Context) -> BoxResult {
        let api = Api::new(ctx);
        api.run().await?;
        Ok(())
    }
}
