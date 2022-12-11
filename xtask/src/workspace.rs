/*
    Appellation: wasm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Workspace {
    pub root: PathBuf,
}

impl Workspace {
    pub fn new() -> Self {
        let root = Default::default();
        Self { root }
    }
    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }
    pub fn update_root(&mut self, path: PathBuf) -> &Self {
        self.root = path;
        self
    }
}
