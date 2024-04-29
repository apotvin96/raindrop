use bevy_ecs::component::Component;

#[derive(Component)]
pub struct AudioSource {
    pub id: String,
    pub spatial: bool,
}
