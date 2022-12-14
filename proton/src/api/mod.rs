/*
    Appellation: api <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::utils::*;


pub(crate) mod utils {
    use gloo::net::http::Request;
    use wasm_bindgen::prelude::*;

    ///
    #[wasm_bindgen]
    pub async fn fetch(path: &str) -> Result<String, JsError> {
        let resp = Request::get(path).send().await?;

        Ok(resp.text().await?)
    }

}