use crate::engine::components::{Player, Transform};
use crate::engine::resources::{ControlInput, Time};
use bevy_ecs::system::{Query, Res};
use winit::event::VirtualKeyCode;

pub fn player_control_system(
    mut query: Query<(&mut Transform, &Player)>,
    control_input: Res<ControlInput>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut query {
        let current_rotation = transform.get_rotation();

        let mut translation = glm::Vec3::zeros();
        let mut rotation = glm::Vec3::zeros();

        if control_input.pressed(VirtualKeyCode::W) {
            translation.x +=
                current_rotation.y.cos() * player.get_movement_speed() * time.delta_time;
            translation.z +=
                current_rotation.y.sin() * player.get_movement_speed() * time.delta_time;
        }
        if control_input.pressed(VirtualKeyCode::S) {
            translation.x -=
                current_rotation.y.cos() * player.get_movement_speed() * time.delta_time;
            translation.z -=
                current_rotation.y.sin() * player.get_movement_speed() * time.delta_time;
        }
        if control_input.pressed(VirtualKeyCode::A) {
            translation.x +=
                current_rotation.y.sin() * player.get_movement_speed() * time.delta_time;
            translation.z -=
                current_rotation.y.cos() * player.get_movement_speed() * time.delta_time;
        }
        if control_input.pressed(VirtualKeyCode::D) {
            translation.x -=
                current_rotation.y.sin() * player.get_movement_speed() * time.delta_time;
            translation.z +=
                current_rotation.y.cos() * player.get_movement_speed() * time.delta_time;
        }
        if control_input.pressed(VirtualKeyCode::Q) {
            translation.y -= player.get_movement_speed() * time.delta_time;
        }
        if control_input.pressed(VirtualKeyCode::E) {
            translation.y += player.get_movement_speed() * time.delta_time;
        }

        if control_input.pressed(VirtualKeyCode::Left) {
            rotation.y -= player.get_rotation_speed() * time.delta_time;
        }
        if control_input.pressed(VirtualKeyCode::Right) {
            rotation.y += player.get_rotation_speed() * time.delta_time;
        }

        transform.translate(translation);
        transform.rotate(rotation)
    }
}
