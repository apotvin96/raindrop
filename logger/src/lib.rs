use std::fs::File;

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TargetPadding, TermLogger, TerminalMode, WriteLogger};

pub fn init_logging(log_path: &str) {
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
            File::create(log_path).unwrap(),
        ),
    ])
    .unwrap();
}
