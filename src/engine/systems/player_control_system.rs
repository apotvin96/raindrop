use crate::engine::components::{Camera, Player};
use crate::engine::resources::ControlInput;
use bevy_ecs::{
    query::With,
    system::{Query, Res},
};
use winit::event::VirtualKeyCode;

pub fn player_control_system(
    mut query: Query<&mut Camera, With<Player>>,
    control_input: Res<ControlInput>,
) {
    for mut camera in &mut query {
        let mut translation = glm::vec3(0.0, 0.0, 0.0);

        if control_input.pressed(VirtualKeyCode::W) {
            translation.z -= 0.1;
        }
        if control_input.pressed(VirtualKeyCode::S) {
            translation.z += 0.1;
        }
        if control_input.pressed(VirtualKeyCode::A) {
            translation.x -= 0.1;
        }
        if control_input.pressed(VirtualKeyCode::D) {
            translation.x += 0.1;
        }
        if control_input.pressed(VirtualKeyCode::Q) {
            translation.y -= 0.1;
        }
        if control_input.pressed(VirtualKeyCode::E) {
            translation.y += 0.1;
        }

        camera.translate(translation);
    }
}
