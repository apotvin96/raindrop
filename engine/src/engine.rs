use std::f32::consts::PI;

use bevy_ecs::{schedule::Schedule, world::World};
use config::Config;
use log::trace;
use winit::{event::{Event, VirtualKeyCode, WindowEvent}, window::Window};

use crate::{components::{Camera, Material, Mesh, Player, Transform}, resources::{self, ControlInput}, systems};

pub struct Engine {
    is_initialized: bool,
    world: World,
    update_schedule: Schedule,
    render_schedule: Schedule,
}

impl Engine {
    pub fn new(config: &Config, window: &winit::window::Window) -> Result<Engine, String> {
        let mut world = World::new();

        world.insert_resource(resources::ControlInput::default());
        world.insert_resource(resources::Time::new());
        if !cfg!(test) {
            world.insert_non_send_resource(resources::RendererResource::new(config, window));
        }

        let mut update_schedule = Schedule::default();
        update_schedule.add_systems(systems::player_control_system::player_control_system);
        update_schedule.add_systems(systems::spin_system::spin_system);

        let mut render_schedule = Schedule::default();

        if !cfg!(test) {
            render_schedule.add_systems(systems::renderer_system::renderer_system);
        }

        world.spawn((
            Camera::new(
                (config.renderer.window_width as f32) / (config.renderer.window_height as f32),
                PI / 2.0,
                0.1,
                100.0,
            ),
            Transform::new(),
            Player::new(),
        ));

        world.spawn((
            Transform::new(),
            Mesh {
                id: "monkey".to_string(),
            },
            Material {
                id: "defaultmesh".to_string(),
            },
        ));

        for x in -10..10 {
            for y in -10..10 {
                let mut transform = Transform::new();
                transform.set_translation(glm::vec3(x as f32 * 2.0, 0.0, y as f32 * 2.0));
                transform.set_scale(glm::vec3(0.2, 0.2, 0.2));

                let mesh_str = if y % 2 == 0 { "monkey" } else { "monkey2" };

                world.spawn((
                    transform,
                    Mesh {
                        id: mesh_str.to_string(),
                    },
                    Material {
                        id: "defaultmesh".to_string(),
                    },
                ));
            }
        }

        Ok(Engine {
            is_initialized: true,
            world,
            update_schedule,
            render_schedule,
        })
    }

    pub fn update(&mut self, delta_time: f64) {
        trace!("Engine Updating");

        let mut time = self.world.get_resource_mut::<resources::Time>().unwrap();
        time.delta_time = delta_time as f32;

        self.update_schedule.run(&mut self.world);
    }

    pub fn render(&mut self, _window: &Window) {
        trace!("Engine Rendering");

        self.render_schedule.run(&mut self.world)
    }

    pub fn handle_event(&mut self, event: &winit::event::Event<()>) -> bool {
        trace!("Engine Eventing");

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
