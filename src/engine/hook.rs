use winit::{event_loop::EventLoop, window::WindowBuilder};

use game_loop::game_loop;

use super::Engine;
use crate::config::Config;

pub fn hook(config: Config) {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(config.info.title.clone())
        .with_inner_size(winit::dpi::LogicalSize::new(
            config.renderer.window_width,
            config.renderer.window_height,
        ))
        .build(&event_loop)
        .unwrap();

    let engine = Engine::new(config, &window).expect("Failed to init engine");

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
