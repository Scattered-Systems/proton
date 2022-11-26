/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use wasm_bindgen::JsError;

pub type WasmError = JsError;
pub type WasmResult<R = (), E = WasmError> = Result<R, E>;
