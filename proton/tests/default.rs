#[cfg(test)]
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn lib_compiles() {
    let f = |i: usize| i + 1;

    assert_eq!(f(10), 11)
}
