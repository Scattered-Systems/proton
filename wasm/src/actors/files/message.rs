/*
    Appellation: message <files>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Chunks;
use gloo::file::File;

pub enum Msg {
    Loaded(String, String),
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>, Chunks),
    ToggleReadBytes,
}
