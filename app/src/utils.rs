/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
#[cfg(feature = "wasm")]
pub use self::wasm::*;

pub mod auth {

    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

    pub fn get_auth_header(token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers
    }
}

#[cfg(feature = "wasm")]
mod wasm {
    use crate::JsResult;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode, Response};

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
}
