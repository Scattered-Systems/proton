/*
    Appellation: primitives <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
*/
pub use self::{constants::*, statics::*, types::*};

mod constants {}

mod statics {}

mod types {
    /// Type alias for [axum::Json] with a default set equal to [serde_json::Value]
    pub type AxumJson<T = serde_json::Value> = axum::Json<T>;
}
