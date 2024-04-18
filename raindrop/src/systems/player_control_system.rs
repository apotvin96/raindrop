use crate::components::{Player, Transform};
use crate::resources::{ControlInput, Time};
use bevy_ecs::system::{Query, Res};
use winit::event::VirtualKeyCode;

pub fn player_control_system(
    mut query: Query<(&mut Transform, &mut Player)>,
    control_input: Res<ControlInput>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut query {
        let current_rotation = transform.get_rotation();

        let mut translation = glm::Vec3::zeros();
        let mut rotation = glm::Vec3::zeros();

        if control_input.pressed(VirtualKeyCode::LShift) {
            player.set_movement_speed(10.0);
        } else {
            player.set_movement_speed(5.0);
        }

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
