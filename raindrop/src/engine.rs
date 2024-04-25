use bevy_ecs::{
    schedule::{IntoSystemConfigs, Schedule},
    world::World,
};
use config::Config;
use log::trace;
use winit::{
    event::{Event, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::{
    resources::{AssetManagerResource, ControlInput, GameConfig, RendererResource},
    systems, Time,
};

pub enum ScheduleType {
    Startup,
    Update,
    Render,
}

pub struct Engine {
    world: World,
    startup_schedule: Schedule,
    update_schedule: Schedule,
    render_schedule: Schedule,
    shutdown_schedule: Schedule,
}

impl Engine {
    pub fn new(config: &Config, window: &winit::window::Window) -> Result<Engine, String> {
        let world = Engine::default_world(config, window);

        let startup_schedule = Engine::default_startup_schedule();
        let update_schedule = Engine::default_update_schedule();
        let render_schedule = Engine::default_render_schedule();
        let shutdown_schedule = Engine::default_shutdown_schedule();

        let engine = Engine {
            world,
            startup_schedule,
            update_schedule,
            render_schedule,
            shutdown_schedule,
        };

        Ok(engine)
    }

    pub fn add_systems<M>(
        &mut self,
        schedule_type: ScheduleType,
        systems: impl IntoSystemConfigs<M>,
    ) {
        match schedule_type {
            ScheduleType::Startup => {
                self.startup_schedule.add_systems(systems);
            }
            ScheduleType::Update => {
                self.update_schedule.add_systems(systems);
            }
            ScheduleType::Render => {
                self.render_schedule.add_systems(systems);
            }
        }
    }

    pub fn startup(&mut self) {
        trace!("Engine Starting");

        let mut asset_manager_resource = self
            .world
            .get_resource_mut::<AssetManagerResource>()
            .unwrap();

        let mesh_path = "assets/models/monkey/monkey.glb".to_string();
        asset_manager_resource.asset_manager.get_mesh(&mesh_path);

        self.startup_schedule.run(&mut self.world);
    }

    pub fn update(&mut self, delta_time: f64) {
        trace!("Engine Updating");

        let mut time = self.world.get_resource_mut::<Time>().unwrap();
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
                    event:
                        winit::event::KeyEvent {
                            physical_key,
                            state,
                            ..
                        },
                    ..
                } => match (physical_key, state) {
                    (PhysicalKey::Code(KeyCode::Escape), _) => {
                        return false;
                    }
                    (physical_key, state) => {
                        if let PhysicalKey::Code(keycode) = physical_key {
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
                        };
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

    pub fn cleanup(&mut self) {
        trace!("Cleaning");

        self.shutdown_schedule.run(&mut self.world);
    }

    fn default_world(config: &Config, window: &Window) -> World {
        let mut world = World::new();

        world.insert_resource(AssetManagerResource::default());
        world.insert_resource(GameConfig::from(config.clone()));
        world.insert_resource(ControlInput::default());
        world.insert_resource(Time::new());
        world.insert_non_send_resource(RendererResource::new(config.clone(), window));

        world
    }

    fn default_startup_schedule() -> Schedule {
        Schedule::default()
    }

    fn default_update_schedule() -> Schedule {
        let mut schedule = Schedule::default();

        schedule.add_systems(systems::player_control_system);
        schedule.add_systems(systems::spin_system);

        schedule
    }

    fn default_render_schedule() -> Schedule {
        let mut schedule = Schedule::default();

        schedule.add_systems(systems::renderer_system);

        schedule
    }

    fn default_shutdown_schedule() -> Schedule {
        let mut schedule = Schedule::default();

        schedule.add_systems(systems::renderer_shutdown_system);

        schedule
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.cleanup();
    }
}
