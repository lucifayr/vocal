use clap::Parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use input::args::Args;
use input::config::Config;
use log::LevelFilter;
use logging::create_log_file;
use run::run;
use simplelog::WriteLogger;

mod audio;
mod events;
mod input;
mod instance;
mod logging;
mod render;
mod run;
mod state;

fn main() -> Result<(), String> {
    let config = match confy::load("vocal", "config") {
        Ok(config) => config,
        Err(_) => {
            let config = Config::default();
            match confy::store("vocal", "config", config.clone()) {
                Ok(_) => config,
                Err(err) => {
                    return Err(format!("Failed to load config: {err}"));
                }
            }
        }
    };

    match enable_raw_mode() {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("Failed to enable raw keyboard mod: {err}"));
        }
    }

    let log_file = match create_log_file(&config.log_directory, &config.log_file_prefix) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!("Failed to create log file: {err}"));
        }
    };

    match WriteLogger::init(LevelFilter::Info, simplelog::Config::default(), log_file) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("Failed to initialize logger: {err}"));
        }
    }

    run(config, Args::parse())?;

    match disable_raw_mode() {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("Failed to disable raw keyboard mod: {err}"));
        }
    }

    Ok(())
}
