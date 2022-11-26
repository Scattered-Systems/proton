#[cfg(test)]
mod tests {
    use proton::add_one;
    use proton::api::github::fetch_github;
    use wasm_bindgen_test::wasm_bindgen_test;

    // #[tokio::test]
    #[wasm_bindgen_test]
    async fn test_fetch() {
        let url = "https://api.exchange.coinbase.com/currencies";
        let a = fetch_github("scattered-systems/proton".to_string()).await;

        assert!(a.is_err())
    }

    #[wasm_bindgen_test]
    fn lib_compiles() {
        assert_eq!(add_one(10), 11)
    }
}
