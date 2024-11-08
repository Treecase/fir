//! A minimalist image viewer for Wayland.

use cli::args::{parse, Request};
use cli::print;
use config::Config;
use std::env;
use std::error::Error;

mod cli;
mod config;
mod gui;
mod meta;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = env::args().skip(1);
    let request = parse(args);

    let request = match request {
        Err(err) => {
            println!("error: {err}");
            print::usage();
            return Ok(());
        }
        Ok(r) => r,
    };

    // Handle decoded arguments.
    let files = match request {
        Request::Help => {
            print::help();
            return Ok(());
        }
        Request::Version => {
            print::version();
            return Ok(());
        }
        Request::View { files } => files,
    };

    let config = Config::from_config_toml();

    gui::start(files, config)
}
