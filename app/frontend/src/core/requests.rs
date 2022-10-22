/*
    Appellation: requests <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{data::meta::ErrorInfo, errors::Error, API_ROOT, TOKEN, TOKEN_KEY};
use gloo::{
    net::http::{Method, Request},
    storage::{LocalStorage, Storage},
};

use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;
use web_sys::RequestCredentials;

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    if let Some(t) = token.clone() {
        LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

/// build all kinds of http request: post/get/delete etc.
pub async fn request<B, T>(method: Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body =
        method.to_string() == "POST".to_string() || method.to_string() == "PUT".to_string();
    let url = format!("{}{}", API_ROOT, url);
    let mut builder = Request::new(url.as_str())
        .method(method)
        .header("Content-Type", "application/json");
    if let Some(token) = get_token() {
        let token = JsValue::from_str(token.as_str());
        let credentials = RequestCredentials::from_js_value(&token).expect("");
        builder = builder.credentials(credentials);
    }
    if allow_body {
        builder = builder.json(&body).expect("");
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.ok() == true {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                log::debug!("Response: {:?}", data);
                Ok(data)
            } else {
                Err(Error::DeserializeError)
            }
        } else {
            match data.status() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => {
                    let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}

/// Delete request
pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(Method::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(Method::GET, url, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(Method::POST, url, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(Method::PUT, url, body).await
}

/// Set limit for pagination
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
