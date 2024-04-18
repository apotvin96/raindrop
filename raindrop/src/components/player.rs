use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Player {
    /// Movement speed in units per second
    movement_speed: f32,
    /// Rotation speed in radians per second
    rotation_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            movement_speed: 5.0,
            rotation_speed: 3.0,
        }
    }

    pub fn set_movement_speed(&mut self, movement_speed: f32) {
        self.movement_speed = movement_speed;
    }

    pub fn get_movement_speed(&self) -> f32 {
        self.movement_speed
    }

    pub fn set_rotation_speed(&mut self, rotation_speed: f32) {
        self.rotation_speed = rotation_speed;
    }

    pub fn get_rotation_speed(&self) -> f32 {
        self.rotation_speed
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_player_new() {
        let player = super::Player::new();

        assert_eq!(player.movement_speed, 5.0);
        assert_eq!(player.rotation_speed, 3.0);
    }

    #[test]
    fn test_movement_speed_set() {
        let mut player = super::Player::new();

        player.set_movement_speed(10.0);

        assert_eq!(player.movement_speed, 10.0);
    }

    #[test]
    fn test_movement_speed_get() {
        let player = super::Player::new();

        assert_eq!(player.get_movement_speed(), 5.0);
    }

    #[test]
    fn test_rotation_speed_set() {
        let mut player = super::Player::new();

        player.set_rotation_speed(6.0);

        assert_eq!(player.rotation_speed, 6.0);
    }

    #[test]
    fn test_rotation_speed_get() {
        let player = super::Player::new();

        assert_eq!(player.get_rotation_speed(), 3.0);
    }
}
