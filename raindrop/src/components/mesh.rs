use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Mesh {
    // TODO: This needs to become a number or some cheaply comparable and copyable type
    pub id: String,
}
