use crate::engine::components::{Mesh, Transform};
use crate::engine::resources::Time;
use bevy_ecs::query::With;
use bevy_ecs::system::{Query, Res};

pub fn spin_system(mut query: Query<&mut Transform, With<Mesh>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate(glm::vec3(0.0, 0.0, time.delta_time))
    }
}
