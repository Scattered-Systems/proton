#[cfg(test)]
mod tests {
    use proton::add_one;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_fetch() {
        let a = proton::api::fetch_json_simple("https://api.exchange.coindbase.com/currencies");
        
    }

    #[wasm_bindgen_test]
    fn lib_compiles() {
        assert_eq!(add_one(10), 11)
    }
}
