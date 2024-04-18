use engine::{logging, raindrop, Config};

fn main() {
    logging::init_logging("engine.log");

    let config = Config::from_file("game_config.toml");

    let mut raindrop = raindrop::Raindrop::new(config);

    raindrop.run();
}
