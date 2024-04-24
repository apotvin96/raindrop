use crate::components::{Player, Transform};
use crate::resources::{ControlInput, Time};
use bevy_ecs::system::{Query, Res};
use winit::keyboard::KeyCode;

pub fn player_control_system(
    mut query: Query<(&mut Transform, &mut Player)>,
    control_input: Res<ControlInput>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut query {
        let current_rotation = transform.get_rotation();

        let mut translation = glm::Vec3::zeros();
        let mut rotation = glm::Vec3::zeros();

        if control_input.pressed(KeyCode::ShiftLeft) {
            player.set_movement_speed(10.0);
        } else {
            player.set_movement_speed(5.0);
        }

        if control_input.pressed(KeyCode::KeyW) {
            translation.x +=
                current_rotation.y.cos() * player.get_movement_speed() * time.delta_time;
            translation.z +=
                current_rotation.y.sin() * player.get_movement_speed() * time.delta_time;
        }
        if control_input.pressed(KeyCode::KeyS) {
            translation.x -=
                current_rotation.y.cos() * player.get_movement_speed() * time.delta_time;
            translation.z -=
                current_rotation.y.sin() * player.get_movement_speed() * time.delta_time;
        }
        if control_input.pressed(KeyCode::KeyA) {
            translation.x +=
                current_rotation.y.sin() * player.get_movement_speed() * time.delta_time;
            translation.z -=
                current_rotation.y.cos() * player.get_movement_speed() * time.delta_time;
        }
        if control_input.pressed(KeyCode::KeyD) {
            translation.x -=
                current_rotation.y.sin() * player.get_movement_speed() * time.delta_time;
            translation.z +=
                current_rotation.y.cos() * player.get_movement_speed() * time.delta_time;
        }
        if control_input.pressed(KeyCode::KeyQ) {
            translation.y -= player.get_movement_speed() * time.delta_time;
        }
        if control_input.pressed(KeyCode::KeyE) {
            translation.y += player.get_movement_speed() * time.delta_time;
        }

        if control_input.pressed(KeyCode::ArrowLeft) {
            rotation.y -= player.get_rotation_speed() * time.delta_time;
        }
        if control_input.pressed(KeyCode::ArrowRight) {
            rotation.y += player.get_rotation_speed() * time.delta_time;
        }

        transform.translate(translation);
        transform.rotate(rotation)
    }
}
