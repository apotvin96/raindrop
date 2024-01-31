use std::sync::Arc;

use winit::{event_loop::EventLoop, window::Window};

use game_loop::game_loop;

use crate::config::Config;
use crate::engine::Engine;

pub fn hook(config: Config) {
    let event_loop = EventLoop::new();

    let window = Window::new(&event_loop).unwrap();

    let window = Arc::new(window);

    let mut engine = Engine::new(config, &window).expect("Failed to init engine");

    game_loop(
        event_loop,
        window,
        engine,
        60,
        0.1,
        |g| {
            g.game.update();
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
