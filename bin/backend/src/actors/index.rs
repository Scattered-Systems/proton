/*
    Appellation: index <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use axum::{routing::get, Json, Router};
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BaseRouter(String);

impl BaseRouter {
    pub fn new() -> Self {
        Self("/".to_string())
    }
    pub fn router(&self) -> Router {
        Router::new().route("/", get(landing))
    }
}

impl Default for BaseRouter {
    fn default() -> Self {
        Self("/".to_string())
    }
}

pub async fn landing() -> Json<Value> {
    let timestamp = Timestamp::default();

    Json(json!({ "timestamp": timestamp }))
}
