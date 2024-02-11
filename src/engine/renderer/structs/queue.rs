use ash::{
    vk::{self, PhysicalDevice, Semaphore},
    Device, Instance,
};
use log::warn;

use super::{Surface, Swapchain};

#[derive(Clone)]
pub struct Queue {
    pub main_queue_index: u32,
    pub transfer_only_queue_index: u32,
    pub main_queue: vk::Queue,
    pub transfer_only_queue: vk::Queue,
}

const NONE_QUEUE_INDEX: u32 = 999999;

impl Queue {
    pub fn new(
        device: &Device,
        main_queue_index: u32,
        transfer_only_queue_index: u32,
    ) -> Result<Queue, String> {
        Ok(Queue {
            main_queue_index,
            transfer_only_queue_index,
            main_queue: unsafe { device.get_device_queue(main_queue_index, 0) },
            transfer_only_queue: unsafe { device.get_device_queue(transfer_only_queue_index, 0) },
        })
    }

    pub fn get_queue_indicies(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        surface: &Surface,
    ) -> Result<[u32; 2], String> {
        let properties =
            unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };

        let mut main_queue_index = NONE_QUEUE_INDEX;
        let mut transfer_only_queue_index = NONE_QUEUE_INDEX;

        for (index, queue_family_properties) in properties.iter().enumerate() {
            let queue_supports_surface = surface
                .is_queue_family_supported(physical_device, index as u32)
                .unwrap();

            if queue_family_properties.queue_count == 0 {
                continue;
            }

            if queue_family_properties
                .queue_flags
                .contains(vk::QueueFlags::GRAPHICS)
                && queue_supports_surface
            {
                main_queue_index = index as u32;
            }

            if queue_family_properties
                .queue_flags
                .contains(vk::QueueFlags::TRANSFER)
            {
                // If we haven't found a transfer queue yet, or we found one that has only a transfer queue prefer that
                //   The reason is that a having separate queue for transfer and graphics is preferred for performance
                //   but not guaranteed in a system
                if transfer_only_queue_index == NONE_QUEUE_INDEX
                    || !queue_family_properties
                        .queue_flags
                        .contains(vk::QueueFlags::GRAPHICS)
                {
                    transfer_only_queue_index = index as u32;
                }
            }
        }

        if main_queue_index == NONE_QUEUE_INDEX || transfer_only_queue_index == NONE_QUEUE_INDEX {
            return Err("Failed to find suitable queues".to_owned());
        }

        if main_queue_index == transfer_only_queue_index {
            warn!("Using the same queue for graphics and transfer operations")
        }

        Ok([main_queue_index, transfer_only_queue_index])
    }
}
