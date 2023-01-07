/*
    Appellation: utils <modules>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Bundle;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::{fs, io, process::Command};

pub fn program(program: &str, args: &[&str]) -> Result<()> {
    let mut cmd = Command::new(program);
    cmd.current_dir(project_root());
    cmd.args(args).status()?;
    Ok(())
}

pub fn dist_dir() -> PathBuf {
    project_root().join(".artifacts/dist")
}
/// Execute a collection of commands 
pub fn execute_bundle(bundle: Bundle) -> Result<()> {
    for k in bundle.keys() {
        for i in 0..bundle[k].len() {
            program(k, bundle[k][i].clone().as_slice())?;
        }
    }
    Ok(())
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
pub fn package_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

/// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
