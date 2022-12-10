/*
    Appellation: tag <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{data::meta::*, errors::Error, request_get};

/// Get all tags
pub async fn get_all() -> Result<TagListInfo, Error> {
    request_get::<TagListInfo>("/tags".to_string()).await
}
