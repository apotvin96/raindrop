use bevy_ecs::system::{NonSendMut, ResMut};

use crate::resources::{AssetManagerResource, RendererResource};

pub fn renderer_shutdown_system(
    mut renderer: NonSendMut<RendererResource>,
    mut asset_manager: ResMut<AssetManagerResource>,
) {
    renderer
        .as_mut()
        .renderer
        .cleanup(&mut asset_manager.as_mut().asset_manager);
}
