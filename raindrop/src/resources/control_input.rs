use std::collections::HashMap;

use bevy_ecs::system::Resource;
use winit::keyboard::KeyCode;

#[derive(Resource, Default)]
pub struct ControlInput {
    keys: HashMap<KeyCode, bool>,
}

impl ControlInput {
    pub fn set_key_down(&mut self, key: KeyCode) {
        self.keys.insert(key, true);
    }

    pub fn set_key_up(&mut self, key: KeyCode) {
        self.keys.insert(key, false);
    }

    pub fn pressed(&self, key: KeyCode) -> bool {
        match self.keys.get(&key) {
            Some(value) => *value,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_input_pressed() {
        let mut control_input = ControlInput::default();

        control_input.set_key_down(KeyCode::KeyW);
        assert!(control_input.keys.contains_key(&KeyCode::KeyW));
    }

    #[test]
    fn test_control_input_released() {
        let mut control_input = ControlInput::default();

        control_input.set_key_up(KeyCode::KeyW);
        assert!(!control_input.pressed(KeyCode::KeyW));
    }

    #[test]
    fn test_control_input_false_if_not_present() {
        let control_input = ControlInput::default();

        assert!(!control_input.pressed(KeyCode::KeyW));
    }

    #[test]
    fn test_control_input_true_if_present() {
        let mut control_input = ControlInput::default();

        control_input.set_key_down(KeyCode::KeyW);
        assert!(control_input.pressed(KeyCode::KeyW));
    }

    #[test]
    fn test_control_input_updates_key() {
        let mut control_input = ControlInput::default();

        control_input.set_key_down(KeyCode::KeyW);
        assert!(control_input.pressed(KeyCode::KeyW));

        control_input.set_key_up(KeyCode::KeyW);
        assert!(!control_input.pressed(KeyCode::KeyW));
    }
}
