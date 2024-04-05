use winit::window::Window;

use crate::{config, engine::renderer::Renderer};

pub struct RendererResource {
    pub renderer: Renderer,
}

impl RendererResource {
    pub fn new(config: &config::Config, window: &Window) -> Self {
        let renderer  = match Renderer::new(config, &window) {
            Ok(renderer) => renderer,
            Err(e) => panic!("Failed to init renderer: {}", e),
        };

        Self { renderer }
    }
}
