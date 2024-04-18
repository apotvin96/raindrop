use bevy_ecs::schedule::IntoSystemConfigs;
use config::Config;
use logger::init_logging;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::engine::{Engine, ScheduleType};

use game_loop::game_loop;

pub struct Raindrop {
    event_loop: Option<EventLoop<()>>,
    window: Option<Window>,
    engine: Option<Engine>,
}

impl Raindrop {
    pub fn empty() -> Raindrop {
        Raindrop {
            event_loop: None,
            window: None,
            engine: None,
        }
    }

    pub fn new(config: &Config) -> Raindrop {
        init_logging("engine.log");

        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title(config.info.name.clone())
            .with_inner_size(winit::dpi::LogicalSize::new(
                config.renderer.window_width,
                config.renderer.window_height,
            ))
            .build(&event_loop)
            .unwrap();

        let engine = Engine::new(config, &window).expect("Failed to init engine");

        Raindrop {
            event_loop: Some(event_loop),
            window: Some(window),
            engine: Some(engine),
        }
    }

    /// Add a system to be run for the engine
    /// Specify the schedule type for the schedule to be added to
    pub fn add_systems<M>(
        &mut self,
        schedule_type: ScheduleType,
        systems: impl IntoSystemConfigs<M>,
    ) {
        self.engine.as_mut().unwrap().add_systems(schedule_type, systems);
    }

    /// Run the game loop
    pub fn run(&mut self) {
        /*
         * Ok so this is fucked but lets explain.
         * We need our app and its data to be owned by the game_loop
         * And, since this is Rust, we have to ensure ownership is safe
         * So,
         *     1. We take our existing app, and do a mem::replace over it
         *     2. mem::replace replaces self with a new empty Raindrop instance
         *     3. The replaced objec is returned by mem replace, effectively giving us a now unowned
         *        instance of Raindrop (owned by the new app var)
         *     4. We hand it now to the game_loop so it can own it
         *
         * I hate this but this is a very close replication of what Bevy does, and they are better than
         * me so im gonna trust them.
         */
        let app = std::mem::replace(self, Raindrop::empty());

        let event_loop = app.event_loop.unwrap();
        let window = app.window.unwrap();
        let mut engine = app.engine.unwrap();

        engine.startup();

        game_loop(
            event_loop,
            window,
            engine,
            60,
            0.1,
            |g| {
                g.game.update(g.fixed_time_step());
            },
            |g| {
                g.game.render(&g.window);
            },
            |g, event| {
                if !g.game.handle_event(event) {
                    g.exit();
                }
            },
        );
    }
}
