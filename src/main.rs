pub mod vk_engine;

use crate::vk_engine::VulkanEngine;

fn main() {
    let mut engine = VulkanEngine::new();

    engine.init();

    engine.run();

    engine.cleanup();
}
