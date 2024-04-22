use asset_manager::AssetManager;
use bevy_ecs::system::Resource;

#[derive(Resource, Default)]
pub struct AssetManagerResource {
    pub asset_manager: AssetManager
}
