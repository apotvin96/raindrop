use std::f32::consts::PI;

use raindrop::{
    bevy_ecs::system::{Commands, Res},
    components::{Camera, Material, Mesh, Player, Transform},
    glm, Config, GameConfig, Raindrop, ScheduleType,
};

fn init_scene(mut commands: Commands, config: Res<GameConfig>) {
    commands.spawn((
        Camera::new(
            (config.config.renderer.window_width as f32)
                / (config.config.renderer.window_height as f32),
            PI / 2.0,
            0.1,
            100.0,
        ),
        Transform::new(),
        Player::new(),
    ));

    commands.spawn((
        Transform::new(),
        Mesh {
            id: "assets/models/monkey/monkey.glb".to_string(),
        },
        Material {
            id: "defaultmesh".to_string(),
        },
    ));

    for x in -10..10 {
        for y in -10..10 {
            let mut transform = Transform::new();
            transform.set_translation(glm::vec3(x as f32 * 2.0, 0.0, y as f32 * 2.0));
            transform.set_scale(glm::vec3(0.2, 0.2, 0.2));

            let mesh_str = if y % 2 == 0 {
                "assets/models/monkey/monkey.glb"
            } else {
                "assets/models/monkey/monkey.glb"
            };

            commands.spawn((
                transform,
                Mesh {
                    id: mesh_str.to_string(),
                },
                Material {
                    id: "defaultmesh".to_string(),
                },
            ));
        }
    }
}

fn main() {
    let config = Config::from_file("game_config.toml");

    let mut raindrop = Raindrop::new(&config);

    raindrop.add_systems(ScheduleType::Startup, init_scene);

    raindrop.run();
}
