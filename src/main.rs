extern crate log;
extern crate simplelog;

use std::fs::File;

use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode, WriteLogger, TargetPadding
};

pub mod config;
pub mod engine;

use config::Config;

use crate::engine::hook;

fn main() {
    let log_config = ConfigBuilder::new()
        .add_filter_ignore(format!("{}", "winit"))
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

    let config = Config::from_file("config.toml");

    hook(config);
}
