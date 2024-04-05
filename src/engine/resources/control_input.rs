use std::collections::HashMap;

use bevy_ecs::system::Resource;
use log::warn;
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
