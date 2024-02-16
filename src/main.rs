extern crate log;
extern crate nalgebra_glm as glm;
extern crate simplelog;

#[macro_use]
extern crate lazy_static;

use std::fs::File;

use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TargetPadding, TermLogger,
    TerminalMode, WriteLogger,
};

pub mod config;
pub mod engine;

use config::Config;
use engine::hook;

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
