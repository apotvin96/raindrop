use crate::engine::{
    components::{Camera, Player, Transform},
    resources::{RendererResource, Time},
};
use bevy_ecs::{
    query::With,
    system::{NonSendMut, Query, Res},
};

pub fn renderer_system(
    mut query: Query<(&mut Camera, &mut Transform), With<Player>>,
    mut renderer: NonSendMut<RendererResource>,
    time: Res<Time>,
) {
    let (camera, mut transform) = query.iter_mut().next().unwrap();

    let view_matrix = transform.matrix();
    let projection_matrix = camera.matrix();

    renderer
        .as_mut()
        .renderer
        .render(time.delta_time, projection_matrix, view_matrix);
}
