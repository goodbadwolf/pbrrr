#![warn(clippy::all, clippy::pedantic)]

use clap::App;
use log::info;

mod logging;

fn main() {
    let clap_app = App::new("PBRRR (PBRT V3 in Rust)");

    logging::setup_logger(clap_app.get_bin_name().unwrap_or("pbrrr"))
        .expect("Failed to setup logging");

    info!("Hello, world!");
}
