use ash::{vk::PhysicalDevice, Device, Instance};
use log::trace;
use vk_mem::{Allocator, AllocatorCreateInfo};

pub fn init_allocator(
    instance: &Instance,
    physical_device: &PhysicalDevice,
    device: &Device,
) -> Result<Allocator, String> {
    trace!("Initializing: Vk Allocator");

    match Allocator::new(AllocatorCreateInfo::new(instance, device, *physical_device)) {
        Ok(allocator) => Ok(allocator),
        Err(e) => Err("Renderer: Failed to create allocator:".to_owned() + &e.to_string()),
    }
}