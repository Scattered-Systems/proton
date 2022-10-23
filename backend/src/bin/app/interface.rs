/*
    Appellation: application <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::{api::Api, Context, Settings};
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RESTBackend {
    pub ctx: Context,
}

impl RESTBackend {
    pub fn new() -> Self {
        let settings = Settings::default();
        let context = Context::new(settings.clone());
        Self { ctx: context }
    }
    pub fn from(settings: Settings) -> Self {
        Self {
            ctx: Context::new(settings),
        }
    }
    pub fn api(&self) -> Api {
        Api::new(self.ctx.clone())
    }
    pub fn with_logging(&self) -> &Self {
        self.ctx.settings.logger.setup();
        self
    }
    pub async fn run(&self) -> BoxResult {
        println!("{}", self.ctx.settings.server.clone());
        self.api().run().await.expect("Interface Error");
        Ok(())
    }
}

impl Default for RESTBackend {
    fn default() -> Self {
        Self::from(Settings::default())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        let actual = f(4, 4);
        let expected: usize = 8;
        assert_eq!(actual, expected)
    }
}
