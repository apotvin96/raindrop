use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Player {}
    }
}
