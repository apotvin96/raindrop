use bevy_ecs::system::Resource;

#[derive(Resource, Default)]
pub struct Time {
    pub delta_time: f32,
}

impl Time {
    pub fn new() -> Time {
        Time { delta_time: 0.0 }
    }
}
