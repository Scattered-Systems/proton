/*
    Appellation: proton <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Proton is a unified interface built as a personal computing platform
*/
pub use self::{actors::*, components::*, core::*, data::*};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;

pub mod api {
    use futures::{future, Future, TryFutureExt};
    use js_sys::Promise;
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::{future_to_promise, JsFuture};
    use web_sys::{Response, Request, RequestInit, RequestMode};

    #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct Msg {
        data: String
    }

    impl std::convert::From<&str> for Msg {
        fn from(data: &str) -> Self {
            Self { data: data.to_string() }
        }
    }

    #[wasm_bindgen]
    pub async fn fetch_github(repo: String) -> Result<JsValue, JsValue> {
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

        // Send the JSON response back to JS.
        Ok(json)
    }

    #[wasm_bindgen]
    pub fn fetch_json_simple(url: &str) -> Promise {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let req = Request::new_with_str_and_init(url, &opts).expect("");
        req.headers().set("Accept", "application/json").expect("");

        let window = web_sys::window().expect("");
        let promise = window.fetch_with_request(&req);

        let future = JsFuture::from(promise)
            .and_then(| r | {
                // `resp_value` is a `Response` object.
                assert!(r.is_instance_of::<Response>());
                let resp: Response = r.dyn_into().expect("");
                resp.json().expect("")
            });

        // Convert this Rust `Future` back into a JS `Promise`.
        future_to_promise(future)
    }
}
