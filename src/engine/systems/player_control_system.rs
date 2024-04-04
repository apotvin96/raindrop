use bevy_ecs::{query::With, system::{Query, Res}};

use crate::engine::components::{Camera, Player};

fn player_control_system(query: Query<&mut Camera, With<Player>>, input: Res<Input>) {
    for mut camera in query.iter_mut() {
        let mut translation = glm::vec3(0.0, 0.0, 0.0);
        let mut rotation = glm::vec3(0.0, 0.0, 0.0);

        if input.pressed(KeyCode::W) {
            translation.z -= 0.1;
        }
        if input.pressed(KeyCode::S) {
            translation.z += 0.1;
        }
        if input.pressed(KeyCode::A) {
            translation.x -= 0.1;
        }
        if input.pressed(KeyCode::D) {
            translation.x += 0.1;
        }
        if input.pressed(KeyCode::Q) {
            translation.y -= 0.1;
        }
        if input.pressed(KeyCode::E) {
            translation.y += 0.1;
        }

        camera.translate(translation);
        camera.rotate(rotation);
    }
}
