/*
    Appellation: bip0039 <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{lang::*, wordlist::*};

pub(crate) mod lang;
pub(crate) mod wordlist;

pub const BIP0039_ENDPOINT: &str = "https://raw.githubusercontent.com/bitcoin/bips/master/bip-0039";
