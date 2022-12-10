/*
   Appellation: index <module>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::Settings;
use axum::{extract::Path, routing::*, Extension, Json, Router};
use proton_sdk::rt::Context;
use scsys::prelude::{Configurable, Message};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Homepage(pub String);

impl Homepage {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn router(&mut self) -> Router {
        Router::new()
            .route("/", get(landing))
            .route("/settings", get(settings::<Settings>))
            .route("/notifications/:id", get(notifications).post(notifications))
    }
}

impl Default for Homepage {
    fn default() -> Self {
        Self::new("/".to_string())
    }
}

/// Define the landing endpoint
pub async fn landing() -> Json<Value> {
    let msg = Message::from("Welcome to Proton");
    Json(json!(msg))
}

/// Implements a notification endpoint
pub async fn notifications(Path(id): Path<usize>) -> Json<Value> {
    let data = json!({ "id": id });
    Json(json!(Message::from(data)))
}

/// Broadcasts the current settings specified by the user for the interface and other technical systems to leverage
pub async fn settings<Cnf: Configurable>(Extension(ctx): Extension<Context<Cnf>>) -> Json<Value> {
    Json(json!(ctx.settings))
}
