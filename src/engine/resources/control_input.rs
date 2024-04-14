use std::collections::HashMap;

use bevy_ecs::system::Resource;
use winit::event::VirtualKeyCode;

#[derive(Resource, Default)]
pub struct ControlInput {
    keys: HashMap<VirtualKeyCode, bool>,
}

impl ControlInput {
    pub fn set_key_down(&mut self, key: VirtualKeyCode) {
        self.keys.insert(key, true);
    }

    pub fn set_key_up(&mut self, key: VirtualKeyCode) {
        self.keys.insert(key, false);
    }

    pub fn pressed(&self, key: VirtualKeyCode) -> bool {
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

        control_input.set_key_down(VirtualKeyCode::W);
        assert!(control_input.keys.contains_key(&VirtualKeyCode::W));
    }

    #[test]
    fn test_control_input_released() {
        let mut control_input = ControlInput::default();

        control_input.set_key_up(VirtualKeyCode::W);
        assert_eq!(control_input.pressed(VirtualKeyCode::W), false);
    }

    #[test]
    fn test_control_input_false_if_not_present() {
        let control_input = ControlInput::default();

        assert_eq!(control_input.pressed(VirtualKeyCode::W), false);
    }

    #[test]
    fn test_control_input_true_if_present() {
        let mut control_input = ControlInput::default();

        control_input.set_key_down(VirtualKeyCode::W);
        assert_eq!(control_input.pressed(VirtualKeyCode::W), true);
    }
    
    #[test]
    fn test_control_input_updates_key() {
        let mut control_input = ControlInput::default();

        control_input.set_key_down(VirtualKeyCode::W);
        assert_eq!(control_input.pressed(VirtualKeyCode::W), true);

        control_input.set_key_up(VirtualKeyCode::W);
        assert_eq!(control_input.pressed(VirtualKeyCode::W), false);
    }
}
