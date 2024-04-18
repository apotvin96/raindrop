use bevy_ecs::system::Resource;
use config::Config;


#[derive(Resource, Default)]
pub struct GameConfig {
    pub config: Config,
}

impl GameConfig {
    pub fn from(config: Config) -> GameConfig {
        GameConfig { config }
    }
}