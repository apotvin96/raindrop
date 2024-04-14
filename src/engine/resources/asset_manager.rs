use std::collections::HashMap;

use bevy_ecs::system::Resource;

use crate::engine::assets::Asset;

#[derive(Resource, Default)]
pub struct AssetManager {
}

impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {
        }
    } 
}
