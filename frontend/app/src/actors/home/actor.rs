/*
    Appellation: actor <home>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Message {
    Loaded,
    ToggleTheme
}

pub struct Homepage {
    msg: Message
    
}

