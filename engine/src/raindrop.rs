use std::borrow::BorrowMut;

use config::Config;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::engine::Engine;

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

    pub fn new(config: Config) -> Raindrop {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title(config.info.name.clone())
            .with_inner_size(winit::dpi::LogicalSize::new(
                config.renderer.window_width,
                config.renderer.window_height,
            ))
            .build(&event_loop)
            .unwrap();

        let engine = Engine::new(&config, &window).expect("Failed to init engine");

        Raindrop {
            event_loop: Some(event_loop),
            window: Some(window),
            engine: Some(engine),
        }
    }

    /**
     * Run the game loop
     *
     */
    pub fn run(&mut self) {
        /*
         * Ok so this is fucked but lets explain.
         * We need our app and its data to be owned by the game_loop
         * And, since this is Rust, we have to ensure ownership is safe
         * SO
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

        game_loop(
            app.event_loop.unwrap(),
            app.window.unwrap(),
            app.engine.unwrap(),
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
