use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Player {
    /// Movement speed in units per second
    pub movement_speed: f32,
    /// Rotation speed in radians per second
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
