use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            movement_speed: 5.0,
            rotation_speed: 3.0,
        }
    }
}
