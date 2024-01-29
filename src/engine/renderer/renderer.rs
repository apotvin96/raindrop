use ash::vk::Extent2D;
use log::trace;

use crate::config::Config;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }

    pub fn init(
        &mut self,
        config: Config,
        window: &winit::window::Window,
        window_extent: Extent2D,
    ) {
        trace!("Initializing: Renderer");
    }

    pub fn render(&mut self) {
        trace!("Rendering");
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        trace!("Cleaning: Renderer");
    }
}
