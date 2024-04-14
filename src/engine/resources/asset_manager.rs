use bevy_ecs::system::Resource;

#[derive(Resource, Default)]
pub struct AssetManager {}

impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {}
    } 
}
