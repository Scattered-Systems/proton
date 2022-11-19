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

pub(crate) mod api {
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
    pub fn fetch_json_simple(url: &str) -> Promise {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let req = Request::new_with_str_and_init(url, &opts).unwrap();
        req.headers().set("Accept", "application/json").unwrap();

        let window = web_sys::window().unwrap();
        let promise = window.fetch_with_request(&req);

        let future = JsFuture::from(promise)
            .and_then(|resp_value| {
                // `resp_value` is a `Response` object.
                assert!(resp_value.is_instance_of::<Response>());
                let resp: Response = resp_value.dyn_into().unwrap();
                resp.json()
            })
            .and_then(|json_value: Promise| {
                // Convert this other `Promise` into a rust `Future`.
                JsFuture::from(json_value)
            })
            .and_then(|json| {
                // Use serde to parse the JSON into a struct.
                let message: Msg = json.into_serde().unwrap();

                // Send the `Branch` struct back to JS as an `Object`.
                future::ok(JsValue::from_serde(&message).unwrap())
            });

        // Convert this Rust `Future` back into a JS `Promise`.
        future_to_promise(future)
    }
}
