use ash::{vk, Device};

use super::Queue;

pub struct CommandManager {
    command_pool: vk::CommandPool,
    graphics_command_buffer: vk::CommandBuffer,
    transfer_command_buffer: vk::CommandBuffer,
}

impl CommandManager {
    pub fn new(device: &Device, queue: &Queue) -> Result<CommandManager, String> {
        let pool_create_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue.main_queue_index)
            .build();

        let command_pool = match unsafe { device.create_command_pool(&pool_create_info, None) } {
            Ok(pool) => pool,
            Err(_) => return Err("Failed to create command pool".to_string()),
        };

        Ok(CommandManager { command_pool })
    }
}
