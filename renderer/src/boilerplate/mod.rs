pub mod allocator;
mod device;
pub mod frame_data;
mod instance;
mod physical_device;

use ash::{
    vk::{DebugUtilsMessengerEXT, PhysicalDevice},
    Device, Entry, Instance,
};

use std::mem::ManuallyDrop;

use config::Config;

use self::{allocator::Allocator, frame_data::FrameData};

use super::primitives::{Queue, Surface, Swapchain};

pub struct Boilerplate {
    pub instance: Instance,
    pub debug_messenger: DebugUtilsMessengerEXT,
    pub debug_loader: ash::ext::debug_utils::Instance,
    pub physical_device: PhysicalDevice,
    pub surface: ManuallyDrop<Surface>,
    pub device: Device,
    pub allocator: ManuallyDrop<Allocator>,
    pub queue: Queue,
    pub frame_data: ManuallyDrop<Vec<FrameData>>,
    pub swapchain: Swapchain,
}

impl Boilerplate {
    pub fn new(config: &Config, window: &winit::window::Window) -> Result<Boilerplate, String> {
        let entry = Entry::linked();

        let (instance, debug_loader, debug_messenger) = instance::init_instance(&entry, window)?;

        let physical_device = physical_device::init_physical_device(&instance)?;

        let surface = Surface::new(&entry, &instance, &physical_device, window)?;

        let queue_indices = Queue::get_queue_indicies(&instance, &physical_device, &surface)?;

        let device = device::init_device(&instance, &physical_device, &queue_indices)?;

        let allocator = Allocator::new(&instance, &physical_device, &device)?;

        let queue = Queue::new(&device, queue_indices[0], queue_indices[1])?;

        let mut frame_data = vec![];
        for _ in 0..config.renderer.frame_overlap {
            frame_data.push(FrameData::new(&device, &queue)?);
        }

        let swapchain = Swapchain::new(config, &instance, &device, &allocator, &surface, &queue)?;

        Ok(Boilerplate {
            instance,
            debug_messenger,
            debug_loader,
            physical_device,
            surface: ManuallyDrop::new(surface),
            device,
            allocator: ManuallyDrop::new(allocator),
            queue,
            frame_data: ManuallyDrop::new(frame_data),
            swapchain,
        })
    }

    pub fn wait_for_fences(&self) {
        unsafe {
            for frame_data in self.frame_data.iter() {
                self.device
                    .wait_for_fences(&[frame_data.render_fence], true, u64::MAX)
                    .unwrap();
                self.device
                    .reset_fences(&[frame_data.render_fence])
                    .unwrap();
            }
        }
    }
}

impl Drop for Boilerplate {
    fn drop(&mut self) {
        unsafe {
            self.swapchain.free(&mut self.allocator);

            ManuallyDrop::drop(&mut self.frame_data);

            ManuallyDrop::drop(&mut self.allocator);
            self.device.destroy_device(None);
            ManuallyDrop::drop(&mut self.surface);
            self.debug_loader
                .destroy_debug_utils_messenger(self.debug_messenger, None);
            self.instance.destroy_instance(None);
        }
    }
}
