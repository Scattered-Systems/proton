pub use self::{constants::*, statics::*, types::*};

mod constants {}

mod statics {}

mod types {
    #[cfg(target_family = "wasm")]
    /// Type alias for a [Result] of type T and error [wasm_bindgen::prelude::JsError]
    pub type JsResult<T = ()> = Result<T, wasm_bindgen::prelude::JsError>;
}
