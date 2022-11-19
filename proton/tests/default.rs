#[cfg(test)]
mod tests {
    use proton::add_one;
    use wasm_bindgen_test::wasm_bindgen_test;

    // #[tokio::test]
    #[wasm_bindgen_test]
    async fn test_fetch() {
        let url = "https://api.exchange.coindbase.com/currencies";
        let a = proton::api::fetch_github("scattered-systems/proton".to_string()).await;

        
    }

    #[wasm_bindgen_test]
    fn lib_compiles() {
        assert_eq!(add_one(10), 11)
    }
}
