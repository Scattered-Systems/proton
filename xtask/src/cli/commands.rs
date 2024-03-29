/*
    Appellation: commands <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{copy_dir_all, dist_dir, execute_bundle, program, project_root, Bundle};
use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use std::process::Command;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, ValueEnum)]
pub enum Target {
    #[default]
    Wasm32UnknownUnknown = 0
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, ValueEnum)]
pub enum Linux {
    Debian,
    #[default]
    Ubuntu
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, ValueEnum)]
pub enum Setup {
    Extras,
    #[default]
    Langspace,
}

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
        extras: bool,
        #[clap(value_enum)]
        linux: Option<Linux>,
    },
    Start {
        #[clap(value_enum)]
        target: Option<Target>,
    },
}

impl Commands {
    pub fn handler(&self, desktop: bool, release: bool) -> Result<&Self> {
        tracing::info!("Processing commands issued to the cli...");
        match self {
            Self::Compile { workspace } => {
                tracing::info!("Compiling the codebase...");
                if std::fs::create_dir_all(&dist_dir()).is_err() {
                    tracing::info!("Clearing out the previous build");
                    std::fs::remove_dir_all(&dist_dir())?;
                    std::fs::create_dir_all(&dist_dir())?;
                }

                if desktop {
                    tracing::info!("Bundling the application for desktop distribution...");
                    compile_desktop(None)?;
                } else {
                    compile_wasm(None)?;
                    compile_js(None)?
                }
            }
            Self::Create { name } => {
                println!("{:?}", name.clone());
            }
            Self::Setup { extras, linux } => {
                tracing::info!("Setting up the environment...");
                setup_langspace(extras.clone())?;
                if desktop {
                    setup_desktop(linux.clone())?;
                }
            }
            Self::Start { target } => {
                tracing::info!("Initializing the application server...");
                if desktop {
                    start_desktop()?;
                } else { 
                    start_application()?;
                }
            }
        };
        Ok(self)
    }
}

pub fn npm(args: Vec<&str>) -> Result<()> {
    let mut cmd = Command::new("npm");
    cmd.current_dir(project_root());
    cmd.args(args.as_slice()).status()?;
    Ok(())
}

///
pub fn compile_desktop(save_as: Option<&str>) -> Result<()> {
    tracing::info!("Building for desktops...");
    program("cargo", &["tauri", "build", "--config", "desktop/tauri.conf.json"])?;

    copy_dir_all(
        &project_root().join("desktop/target/release/bundle"), 
        project_root().join(save_as.unwrap_or(".artifacts/dist/bundle"))
    )?;
    Ok(())
}
///
pub fn compile_js(save_as: Option<&str>) -> Result<()> {
    npm(vec!["run", "build"])?;
    copy_dir_all(
        &project_root().join("client/build"), 
        project_root().join(save_as.unwrap_or(".artifacts/dist/build"))
    )?;
    Ok(())
}
///
pub fn compile_wasm(save_as: Option<&str>) -> Result<()> {
    program("wasm-pack", &["build", "client/wasm"])?;

    copy_dir_all(
        &project_root().join("client/wasm/pkg"), 
        project_root().join(save_as.unwrap_or(".artifacts/dist/wasm"))
    )?;

    Ok(())
}
///
pub fn start_application() -> Result<()> {
    let mut cmd = Command::new("npm");
    cmd.current_dir(project_root());
    cmd.args(&["run", "dev"]).status()?;
    Ok(())
}
///
pub fn start_desktop() -> Result<()> {
    let mut cmds = Bundle::<&str>::new();
    cmds.insert(
        "cargo",
        vec![vec!["tauri", "dev", "--config", "desktop/tauri.conf.json"]]
    );
    execute_bundle(cmds)?;
    Ok(())
}
///
pub fn setup_rust_nightly(extras: bool) -> Result<()> {
    let mut cmds = crate::Bundle::new();
    cmds.insert(
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
    cmds.insert(
        "npm",
        vec![vec!["install", "-g", "wasm-pack"]],
    );
    cmds.insert(
        "npm",
        vec![vec!["install"]],
    );
    if extras {
        cmds.insert(
            "rustup",
            vec![
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
    }
    execute_bundle(cmds)
}
///
pub fn setup_langspace(extras: bool) -> Result<()> {
    setup_rust_nightly(extras)?;
    Ok(())
}
/// Handler for configuring the workspace
pub fn setup_desktop(linux: Option<Linux>) -> Result<()> {
    let mut args = crate::Bundle::new();
    args.insert(
        "cargo",
        vec![vec!["install", "tauri-cli"]],
    );
    //
    if linux.is_some() {
        match linux.unwrap_or_default().clone() as i32 {
            1 => {
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
                            "clang",
                            "cmake",
                            "libgtk-3-dev",
                            "libwebkit2gtk-4.0-dev",
                            "libappindicator3-dev",
                            "librsvg2-dev",
                            "llvm",
                            "patchelf",
                            "protobuf-compiler",
                        ],
                    ],
                );
            },
            _ => {}
        }
    } 
    //
    execute_bundle(args)?;
    Ok(())
}
