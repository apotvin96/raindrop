use bevy_ecs::{
    query::With,
    system::{NonSendMut, Query, Res},
};

use crate::engine::{
    components::{Camera, Player},
    resources::{Time, RendererResource},
};

pub fn renderer_system(
    mut query: Query<&mut Camera, With<Player>>,
    mut renderer: NonSendMut<RendererResource>,
    time: Res<Time>,
) {
    let camera_matrix = query.iter_mut().next().unwrap().view_matrix();

    renderer.as_mut().renderer.render( time.delta_time, camera_matrix);
}
