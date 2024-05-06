use ash::{
    vk::{DescriptorSet, Fence, FenceCreateFlags, FenceCreateInfo, Semaphore, SemaphoreCreateInfo},
    Device,
};

use crate::primitives::{CommandManager, Queue};

pub struct FrameData {
    device: Device,

    pub present_semaphore: Semaphore,
    pub render_semaphore: Semaphore,
    pub render_fence: Fence,

    pub command_manager: CommandManager,

    global_descriptor: DescriptorSet,
    pass_descriptor: DescriptorSet,
    material_descriptor: DescriptorSet,
    object_descriptor: DescriptorSet,
}

impl FrameData {
    pub fn new(device: &Device, queue: &Queue) -> Result<FrameData, String> {
        let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);

        let render_fence = match unsafe { device.create_fence(&fence_create_info, None) } {
            Ok(fence) => fence,
            Err(e) => return Err("Failed to create fence: ".to_owned() + &e.to_string()),
        };

        let semaphore_create_info = SemaphoreCreateInfo::default();

        let render_semaphore =
            match unsafe { device.create_semaphore(&semaphore_create_info, None) } {
                Ok(semaphore) => semaphore,
                Err(e) => return Err("Failed to create semaphore: ".to_owned() + &e.to_string()),
            };

        let present_semaphore =
            match unsafe { device.create_semaphore(&semaphore_create_info, None) } {
                Ok(semaphore) => semaphore,
                Err(e) => return Err("Failed to create semaphore: ".to_owned() + &e.to_string()),
            };

        let command_manager = match CommandManager::new(device, queue) {
            Ok(command_manager) => command_manager,
            Err(e) => return Err("Failed to create command manager: ".to_owned() + &e),
        };

        Ok(FrameData {
            device: device.clone(),
            present_semaphore,
            render_semaphore,
            render_fence,
            command_manager,
            global_descriptor: DescriptorSet::null(),
            pass_descriptor: DescriptorSet::null(),
            material_descriptor: DescriptorSet::null(),
            object_descriptor: DescriptorSet::null(),
        })
    }
}

impl Drop for FrameData {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_semaphore(self.render_semaphore, None);
            self.device.destroy_semaphore(self.present_semaphore, None);
            self.device.destroy_fence(self.render_fence, None);
        }
    }
}
