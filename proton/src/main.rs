use acme;
use chrono;

pub mod app;

fn main() {
    let timestamp: acme::primitives::Clock = chrono::Local::now().into();
    println!("Timestamp: {:#?}", timestamp.to_rfc3339_string());
}