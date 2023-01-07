/*
   Appellation: siwe <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use axum::{extract, routing::get, Json, Router};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub fn router() -> Router {
    Router::new().route("/siwe/nonce", get(generate_nonce))
    // .route("/siwe/verify/", post(validate_message))
}

///
pub async fn generate_nonce() -> Json<Value> {
    let nonce = siwe::generate_nonce();
    let payload = json!({ "nonce": nonce });

    Json(payload)
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageValidation {
    pub message: String,
    pub signature: Vec<u8>,
}

pub async fn validate_message(extract::Json(message): extract::Json<MessageValidation>) -> bool {
    let _res = json!({"nonce": "", "message": message});

    true
}
