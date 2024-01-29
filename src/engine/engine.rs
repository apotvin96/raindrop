use ash::vk::Extent2D;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    window::Window,
};
use log::trace;

use crate::config::Config;

use super::renderer::Renderer;

pub struct Engine {
    is_initialized: bool,
    frame_number: u32,
    stop_rendering: bool,
    window_extent: Extent2D,
    window_resized: bool,
    renderer: Renderer,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            is_initialized: false,
            frame_number: 0,
            stop_rendering: false,
            window_extent: Extent2D {
                width: 800,
                height: 600,
            },
            window_resized: false,
            renderer: Renderer::new(),
        }
    }

    pub fn init(&mut self, config: Config, window: &Window) {
        trace!("Initializing");

        self.renderer.init(config, window, self.window_extent);

        self.is_initialized = true;
    }

    pub fn update(&mut self) {
        trace!("Updating");
    }

    pub fn render(&mut self, window: &Window) {
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

        return true;
    }

    pub fn cleanup(&self) {
        println!("Cleaning");
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if self.is_initialized {
            self.cleanup();
        }
    }
}
