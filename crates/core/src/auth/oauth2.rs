/*
    Appellation: oauth <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Grants {
    #[default]
    ClientCredentials = 0,
}

impl From<Grants> for i64 {
    fn from(data: Grants) -> Self {
        data as i64
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OAuth2Client {
    pub id: String,
    pub secret: String,
}

pub struct AccessToken {
    pub client: OAuth2Client,
    pub authorization: String,
    pub scope: Option<String>,
    pub token: String,
}

/// An asynchronous function wrapper for fetching a given access token
pub async fn get_access_token(
    client_id: &str,
    client_secret: &str,
    auth_url: &str,
    scope: &str,
) -> String {
    // Set up the request body
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("scope", scope);

    // Send the request
    let client = reqwest::Client::new();
    let resp = client
        .post(auth_url)
        .form(&params)
        .send()
        .await
        .expect("Failed to send request");

    // Check the response status
    if resp.status().is_success() {
        // Parse the response body
        let value: Value = resp.json().await.expect("Failed to parse response");
        let access_token = value["access_token"].as_str().unwrap();
        access_token.to_string()
    } else {
        "Error".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_access() {
        let grant = Grants::ClientCredentials;
        assert!(true)
    }
}
