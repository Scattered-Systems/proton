/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{copy_dir_all, dist_dir, execute_bundle, project_root};
use clap::Subcommand;
use duct::cmd;
use proton_sdk::prelude::BoxResult;
use std::process::Command;

#[derive(Clone, Debug, Hash, PartialEq, Subcommand)]
pub enum Commands {
    Compile {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        workspace: bool,
    },
    Create {
        #[clap(long, short, value_parser)]
        name: String,
    },
    Setup {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        linux: bool,
    },
    Start {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        dev: bool,
    },
}

impl Commands {
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Compile { workspace } => {
                compile()?;

                println!("{:?}", workspace.clone());
            }
            Self::Create { name } => {
                println!("{:?}", name.clone());
            }
            Self::Setup { linux } => {
                tracing::info!("Setting up the environment...");
                workspace(linux.clone())?;
            }
            Self::Start { dev } => {
                tracing::info!("Initializing the application server...");
                start(dev.clone())?;
            }
        };
        Ok(self)
    }
}

pub fn compile() -> BoxResult {
    let _ = std::fs::remove_dir_all(&dist_dir());
    std::fs::create_dir_all(&dist_dir())?;

    let cmd = cmd!("trunk")
        .dir(project_root())
        .pipe(cmd!("--config", "proton/Trunk.toml", "build", "--release"))
        .run();

    if !cmd.is_err() {
        tracing::info!("{:?}", "Failed to complete the build...");
        Err("Build failed")?;
    }

    let dst = project_root().join("proton/dist/");

    copy_dir_all(&dst, dist_dir().join("proton"))?;

    eprintln!("Cleaning up binaries");
    let tmp = dist_dir().join("proton").display().to_string();
    // List the contents of the recently created bundle
    Command::new("ls")
        .current_dir(project_root())
        .args(&[tmp.as_str(), "-r"])
        .status()?;

    Ok(())
}

pub fn start(dev: bool) -> BoxResult {
    let mut args = std::collections::HashMap::<&str, Vec<Vec<&str>>>::new();
    args.insert(
        "trunk",
        vec![vec!["--config", "proton/Trunk.toml", "serve"]],
    );
    execute_bundle(args)?;
    Ok(())
}
pub fn workspace(linux: bool) -> BoxResult {
    let mut args = std::collections::HashMap::<&str, Vec<Vec<&str>>>::new();
    if linux {
        args.insert(
            "sudo",
            vec![
                vec!["apt", "update", "-y"],
                vec!["apt", "upgrade", "-y"],
                vec!["apt", "autoremove", "-y"],
                vec![
                    "apt",
                    "install",
                    "-y",
                    "libgtk-3-dev",
                    "libwebkit2gtk-4.0-dev",
                    "libappindicator3-dev",
                    "librsvg2-dev",
                    "patchelf",
                    "protobuf-compiler",
                ],
            ],
        );
    }
    args.insert(
        "cargo",
        vec![vec!["install", "tauri-cli", "trunk", "wasm-bindgen-cli"]],
    );
    args.insert(
        "rustup",
        vec![
            vec!["default", "nightly"],
            vec![
                "target",
                "add",
                "wasm32-unknown-unknown",
                "wasm32-wasi",
                "--toolchain",
                "nightly",
            ],
            vec![
                "component",
                "add",
                "clippy",
                "rustfmt",
                "--toolchain",
                "nightly",
            ],
        ],
    );
    //
    execute_bundle(args)?;
    Ok(())
}
