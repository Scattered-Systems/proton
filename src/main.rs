use chrono;

use acme;

pub mod app;

fn main() {
    let timestamp: String = chrono::Local::now().into();
    println!("Timestamp: {:#?}", timestamp);
}