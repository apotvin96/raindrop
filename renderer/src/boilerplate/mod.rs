mod allocator;
mod device;
mod instance;
mod physical_device;

use ash::{
    vk::{DebugUtilsMessengerEXT, PhysicalDevice},
    Device, Entry, Instance,
};

use std::mem::ManuallyDrop;

use vk_mem::Allocator;

use config::Config;

use super::primitives::{CommandManager, Queue, Surface, Swapchain};

pub struct Boilerplate {
    pub instance: Instance,
    pub debug_messenger: DebugUtilsMessengerEXT,
    pub debug_loader: ash::ext::debug_utils::Instance,
    pub physical_device: PhysicalDevice,
    pub surface: ManuallyDrop<Surface>,
    pub device: Device,
    pub allocator: ManuallyDrop<Allocator>,
    pub queue: Queue,
    pub swapchain: Swapchain,
    pub command_manager: ManuallyDrop<CommandManager>,
}

impl Boilerplate {
    pub fn new(_config: &Config, window: &winit::window::Window) -> Result<Boilerplate, String> {
        let entry = Entry::linked();

        let (instance, debug_loader, debug_messenger) = instance::init_instance(&entry, window)?;

        let physical_device = physical_device::init_physical_device(&instance)?;

        let surface = Surface::new(&entry, &instance, &physical_device, window)?;

        let queue_indices = Queue::get_queue_indicies(&instance, &physical_device, &surface)?;

        let device = device::init_device(&instance, &physical_device, &queue_indices)?;

        let allocator = allocator::init_allocator(&instance, &physical_device, &device)?;

        let queue = Queue::new(&device, queue_indices[0], queue_indices[1])?;

        let swapchain = Swapchain::new(&instance, &device, &allocator, &surface, &queue)?;

        let command_manager = CommandManager::new(&device, &queue)?;

        Ok(Boilerplate {
            instance,
            debug_messenger,
            debug_loader,
            physical_device,
            surface: ManuallyDrop::new(surface),
            device,
            allocator: ManuallyDrop::new(allocator),
            queue,
            swapchain,
            command_manager: ManuallyDrop::new(command_manager),
        })
    }

}

impl Drop for Boilerplate {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.command_manager);
            self.swapchain.free(&mut self.allocator);

            ManuallyDrop::drop(&mut self.allocator);
            self.device.destroy_device(None);
            ManuallyDrop::drop(&mut self.surface);
            self.debug_loader
                .destroy_debug_utils_messenger(self.debug_messenger, None);
            self.instance.destroy_instance(None);
        }
    }
}
