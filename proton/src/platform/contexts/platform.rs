/*
    Appellation: platform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

use futures::Future;
use scsys::prelude::{Configurable, Contextual,State};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tower::Service;



#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Context<Cnf: Configurable, T: Default + Display = serde_json::Value> {
    pub settings: Cnf,
    pub state: State<T>
}

impl<Cnf: Configurable> Context<Cnf> {
    pub fn new(settings: Cnf) -> Self {
        let state = Default::default();
        Self { settings, state }
    }
}

impl<Cnf: Configurable> Contextual for Context<Cnf> {
    type Cnf = Cnf;
    type Ctx = Self;

    fn context(&self) -> &Self::Ctx {
        self
    }
}

impl<Cnf: Configurable> tower::layer::Layer<axum::routing::Route> for Context<Cnf> {
    type Service = Self;

    fn layer(&self, inner: axum::routing::Route) -> Self::Service {
        todo!()
    }
}

impl<Cnf: Configurable> Future for Context<Cnf> {
    type Output = serde_json::Value;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

impl<Cnf: Configurable> std::fmt::Debug for Context<Cnf> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.settings).unwrap())
    }
}

impl<Cnf: Configurable> std::fmt::Display for Context<Cnf> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self.settings).unwrap())
    }
}