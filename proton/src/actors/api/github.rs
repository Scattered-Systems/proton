/*
    Appellation: github <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, Request, RequestInit, RequestMode};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Serialize)]
pub struct Headers(pub HashMap<String, String>);

impl Headers {
    pub fn new() -> Self {
        Self(Default::default())
    }
}


#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, PartialOrd, Serialize)]
pub struct Fetch {
    pub endpoint: String,
    pub headers: Headers
}

impl Fetch {
    pub fn new(endpoint: String, headers: Option<Headers>) -> Self {
        let headers = headers.unwrap_or_default();
        Self { endpoint, headers }
    }
    pub fn request(&self) -> Result<Request, JsValue> {
       Request::new_with_str_and_init(&url, &opts)
    }
}

#[wasm_bindgen]
pub async fn fetch_github(repo: String) -> crate::WasmResult<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.github.com/repos/{}/branches/master", repo);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    println!("{:?}", json.clone());
    // Send the JSON response back to JS.
    Ok(json)
}

// #[wasm_bindgen]
// pub fn fetch_json_simple(url: &str) -> Promise {
//     let mut opts = RequestInit::new();
//     opts.method("GET");
//     opts.mode(RequestMode::Cors);

//     let req = Request::new_with_str_and_init(url, &opts).expect("");
//     req.headers().set("Accept", "application/json").expect("");

//     let window = web_sys::window().expect("");
//     let promise = window.fetch_with_request(&req);

//     let future = JsFuture::from(promise)
//         .and_then(| r | {
//             // `resp_value` is a `Response` object.
//             assert!(r.is_instance_of::<Response>());
//             let resp: Response = r.dyn_into().expect("");
//             resp.json().expect("")
//         });

//     // Convert this Rust `Future` back into a JS `Promise`.
//     future_to_promise(future)
// }