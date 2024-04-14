mod assets;
mod components;
mod renderer;
mod resources;
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

use self::{
    components::{Camera, Player},
    resources::ControlInput,
};

pub struct Engine {
    is_initialized: bool,
    world: World,
    update_schedule: Schedule,
    render_schedule: Schedule,
}

impl Engine {
    pub fn new(config: &Config, window: &winit::window::Window) -> Result<Engine, String> {
        let mut world = World::new();

        world.insert_resource(resources::AssetManager::new());
        world.insert_resource(resources::ControlInput::default());
        world.insert_non_send_resource(resources::RendererResource::new(config, window));
        world.insert_resource(resources::Time::new());

        world.spawn((Camera::new(), Player::new()));

        let mut update_schedule = Schedule::default();
        update_schedule.add_systems(systems::player_control_system::player_control_system);

        let mut render_schedule = Schedule::default();
        render_schedule.add_systems(systems::renderer_system::renderer_system);

        Ok(Engine {
            is_initialized: true,
            world,
            update_schedule,
            render_schedule,
        })
    }

    pub fn update(&mut self, delta_time: f64) {
        trace!("Updating");

        let mut time = self.world.get_resource_mut::<resources::Time>().unwrap();
        time.delta_time = delta_time as f32;

        self.update_schedule.run(&mut self.world);
    }

    pub fn render(&mut self, _window: &Window) {
        trace!("Rendering");

        self.render_schedule.run(&mut self.world)
    }

    pub fn handle_event(&mut self, event: &winit::event::Event<()>) -> bool {
        trace!("Eventing");

        if let Event::WindowEvent {
            event,
            window_id: _,
        } = event
        {
            match event {
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
                    (keycode, state) => {
                        let mut control_input =
                            self.world.get_resource_mut::<ControlInput>().unwrap();

                        match state {
                            winit::event::ElementState::Pressed => {
                                control_input.set_key_down(*keycode);
                            }
                            winit::event::ElementState::Released => {
                                control_input.set_key_up(*keycode);
                            }
                        }
                    }
                },
                WindowEvent::CloseRequested => {
                    return false;
                }
                _ => (),
            }
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
