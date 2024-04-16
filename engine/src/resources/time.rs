use bevy_ecs::system::Resource;

#[derive(Resource, Default)]
pub struct Time {
    /// Delta time in seconds between the last frame and current frame
    pub delta_time: f32,
}

impl Time {
    pub fn new() -> Time {
        Time { delta_time: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_new() {
        let time = Time::new();

        assert_eq!(time.delta_time, 0.0);
    }
}