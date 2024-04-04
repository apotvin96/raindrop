mod components;
mod renderer;
mod systems;

pub mod hook;

use bevy_ecs::{schedule::Schedule, world::World};
pub use hook::hook;

use log::trace;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    window::Window,
};

use crate::config::Config;
use renderer::Renderer;

pub struct Engine {
    is_initialized: bool,
    renderer: Renderer,
    world: World,
    schedule: Schedule,
}

impl Engine {
    pub fn new(config: Config, window: &winit::window::Window) -> Result<Engine, String> {
        let renderer = match Renderer::new(config, window) {
            Ok(renderer) => renderer,
            Err(e) => return Err("Failed to init engine: renderer: ".to_owned() + &e),
        };

        let world = World::new();
        let schedule = Schedule::default();

        Ok(Engine {
            is_initialized: true,
            renderer,
            world,
            schedule,
        })
    }

    pub fn update(&mut self) {
        trace!("Updating");

        self.schedule.run(&mut self.world);
    }

    pub fn render(&mut self, _window: &Window) {
        trace!("Rendering");

        self.renderer.render();
    }

    pub fn handle_event(&mut self, event: &winit::event::Event<()>) -> bool {
        trace!("Eventing");

        match event {
            Event::WindowEvent {
                event,
                window_id: _,
            } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                    ..
                } => match (virtual_code, state) {
                    (VirtualKeyCode::Escape, _) => {
                        return false;
                    }
                    _ => {}
                },
                WindowEvent::CloseRequested => {
                    return false;
                }
                _ => (),
            },
            _ => (),
        }

        true
    }

    pub fn cleanup(&self) {
        trace!("Cleaning");
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if self.is_initialized {
            self.cleanup();
        }
    }
}
