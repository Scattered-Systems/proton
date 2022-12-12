/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{copy_dir_all, dist_dir, execute_bundle, project_root, Bundle};
use clap::{Args, Subcommand, ValueEnum};
use duct::cmd;
use proton_sdk::prelude::BoxResult;
use std::process::Command;

#[derive(Clone, Copy, Debug, Hash, PartialEq, ValueEnum)]
pub enum OperatingSystem {
    Debian,
    MacOS,
    Ubuntu,
    Windows
}

impl Default for OperatingSystem {
    fn default() -> Self {
        Self::Ubuntu
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, ValueEnum)]
pub enum Setup {
    Desktop,
    Extras,
    #[default]
    Langspace,
}

#[derive(Clone, Debug, Hash, PartialEq, Subcommand)]
pub enum Commands {
    Compile {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        desktop: bool,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        workspace: bool,
    },
    Create {
        #[clap(long, short, value_parser)]
        name: String,
    },
    Setup {
        #[clap(value_enum)]
        mode: Option<Setup>,
        #[clap(value_enum)]
        os: Option<OperatingSystem>,
    },
    Start {
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        desktop: bool,
        #[arg(action = clap::ArgAction::SetTrue, long, short)]
        dev: bool,
    },
}

impl Commands {
    pub fn handler(&self) -> BoxResult<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Compile { desktop, workspace } => {
                tracing::info!("Compiling the codebase...");
                compile(desktop.clone(), workspace.clone())?;
            }
            Self::Create { name } => {
                println!("{:?}", name.clone());
            }
            Self::Setup { mode, os } => {
                tracing::info!("Setting up the environment...");
                workspace(mode.clone(), os.clone())?;
            }
            Self::Start { desktop, dev } => {
                tracing::info!("Initializing the application server...");
                start(desktop.clone(), dev.clone())?;
            }
        };
        Ok(self)
    }
}

pub fn compile(desktop: bool, workspace: bool) -> BoxResult {
    let _ = std::fs::remove_dir_all(&dist_dir());
    std::fs::create_dir_all(&dist_dir())?;
    let mut cmds = Bundle::<&str>::new();

    if desktop {
        tracing::info!("Building for desktops...");
        cmds.insert(
            "cargo",
            vec![vec!["tauri", "build", "--config", "proton/Trunk.toml"]]
        );
        let dst = project_root().join("desktop/target/release/bundle/");

        copy_dir_all(&dst, dist_dir().join("bundle"))?;
    } else {
        cmds.insert(
            "trunk",
            vec![vec!["--config", "proton/Trunk.toml", "build", "--release"]]
        );
        let dst = project_root().join("proton/dist/");

        copy_dir_all(&dst, dist_dir().join("proton/"))?;
    }

    let tmp = dist_dir().display().to_string();
    // List the contents of the recently created bundle
    Command::new("ls")
        .current_dir(project_root())
        .args(&[tmp.as_str(), "-r"])
        .status()?;

    Ok(())
}

///
pub fn start(desktop: bool, dev: bool) -> BoxResult {
    let mut args = Bundle::<&str>::new();
    if desktop {
        args.insert(
            "cargo",
            vec![vec!["tauri", "dev", "--config", "desktop/tauri.conf.json"]]
        );
    } else {
        if dev {
            args.insert(
                "trunk",
                vec![vec!["--config", "proton/Trunk.toml", "serve"]],
            );
        } else {
            args.insert(
                "trunk",
                vec![vec!["--config", "proton/Trunk.toml", "serve", "--release"]],
            );
        };
    };
    
    
    execute_bundle(args)?;
    Ok(())
}
/// Handler for configuring the workspace
pub fn workspace(mode: Option<Setup>, os: Option<OperatingSystem>) -> BoxResult {
    let mut args = crate::Bundle::new();
    if os.is_some() {
        match os.unwrap().clone() {
            OperatingSystem::Debian => {},
            OperatingSystem::MacOS => {},
            OperatingSystem::Ubuntu => {
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
            },
            OperatingSystem::Windows => {}
        };
    }
    if mode.is_some() {
        match mode.unwrap().clone() {
            Setup::Desktop => {
                args.insert(
                    "cargo",
                    vec![vec!["install", "tauri-cli"]],
                );
            },
            Setup::Extras => {
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
                    ],
                );
            },
            Setup::Langspace => {
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
                    ],
                );
                args.insert(
                    "cargo",
                    vec![vec!["install", "trunk", "wasm-bindgen-cli"]],
                );
            }
        };
    }
    
    
    
    //
    execute_bundle(args)?;
    Ok(())
}
