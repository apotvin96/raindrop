extern crate log;
extern crate nalgebra_glm as glm;

mod engine;
mod hook;
mod components;
mod resources;
mod systems;

use config::Config;
use hook::hook;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TargetPadding, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;


fn main() {
    let log_config = ConfigBuilder::new()
        .add_filter_ignore("winit".to_string())
        .set_thread_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Off)
        .set_target_padding(TargetPadding::Right(40))
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            log_config.clone(),
            File::create("engine.log").unwrap(),
        ),
    ])
    .unwrap();

    let config = Config::from_file("game_config.toml");

    hook(config);
}
