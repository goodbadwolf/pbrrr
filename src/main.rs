#![warn(clippy::all, clippy::pedantic)]

use log::info;

mod logging;

fn main() {
    logging::setup_logger("pbrt-v3-rust").expect("Failed to setup logging");
    info!("Hello, world!");
}
