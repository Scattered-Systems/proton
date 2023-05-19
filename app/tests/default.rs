#[cfg(test)]
#[test]
fn compiles() {
    let f = |i: usize| i * i;
    assert_eq!(f(2), 4);
}

#[cfg(test)]
#[cfg(target_family = "wasm")]
mod wasm {
    use gambit_ui::fetch_github;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn compiles() {
        let f = |i: usize| i * i;
        assert_eq!(f(2), 4);
    }

    #[wasm_bindgen_test]
    async fn test_fetch() {
        assert!(fetch_github("FL03/pzzld".to_string()).await.is_ok());
    }
}
