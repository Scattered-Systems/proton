/*
    Appellation: Proton <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Proton
//! 
//! Proton is desiged to be a multi-platform portal facilitating a myriad of internet-based experiences empowered by a unique blend of technologies.
//! Proton focuses on providing a seamless experience for users, developers, and content creators alike. Proton is powered by a custom orchestration mechanism,
//! Flow. Flow is a harmonic orchestration mechanism built to effeciently manage the execution of a myriad of tasks, from simple to complex on any number of surfaces.
//! This allows users to enjoy the benefits of blockchain and cloud technologies without the hassle of managing them.
//! 
//! ## Token
//! Proton is distributed as an ERC-6551 token, wrapping pre-existing ENS namespaces enabling users to access the Proton portal anywhere in the world.
//! Owners will enjoy custom wallets, fully-managed application / website hosting furthered with a built-in low-code editor, and a suite of other features.
pub use self::{context::*, settings::*};

mod context;
mod settings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    Ok(())
}