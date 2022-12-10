/*
    Appellation: auth <meta>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub bio: Option<String>,
    pub email: String,
    pub image: Option<String>,
    pub token: String,
    pub username: String,
}

impl UserInfo {
    pub fn is_authenticated(&self) -> bool {
        !self.token.is_empty()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateInfo {
    pub email: String,
    pub username: String,
    pub password: Option<String>,
    pub image: String,
    pub bio: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateInfoWrapper {
    pub user: UserUpdateInfo,
}
