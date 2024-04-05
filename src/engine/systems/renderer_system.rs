use bevy_ecs::{
    query::With,
    system::{NonSendMut, Query},
};

use crate::engine::{
    components::{Camera, Player},
    resources::RendererResource,
};

pub fn renderer_system(
    mut query: Query<&mut Camera, With<Player>>,
    mut renderer: NonSendMut<RendererResource>,
) {
    let camera_matrix = query.iter_mut().next().unwrap().view_matrix();

    renderer.as_mut().renderer.render(camera_matrix);
}
