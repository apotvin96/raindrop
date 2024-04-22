use asset_manager::AssetManager;
use bevy_ecs::system::Resource;

#[derive(Resource, Default)]
pub struct AssetManagerResource {
    asset_manager: AssetManager
}
