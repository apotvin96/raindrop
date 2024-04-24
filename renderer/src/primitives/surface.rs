use ash::{
    vk::{PhysicalDevice, SurfaceKHR},
    Entry, Instance,
};
use winit::window::Window;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct Surface {
    pub surface: SurfaceKHR,
    surface_loader: ash::khr::surface::Instance,
    pub capabilities: ash::vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<ash::vk::SurfaceFormatKHR>,
}

impl Surface {
    pub fn new(
        entry: &Entry,
        instance: &Instance,
        physical_device: &PhysicalDevice,
        window: &Window,
    ) -> Result<Surface, String> {
        let raw_display_handle = match window.display_handle() {
            Ok(handle) => handle.as_raw(),
            Err(_) => return Err("Failed to get raw display handle".to_string()),
        };

        let raw_window_handle = match window.window_handle() {
            Ok(handle) => handle.as_raw(),
            Err(_) => return Err("Failed to get raw window handle".to_string()),
        };

        let surface = match unsafe {
            ash_window::create_surface(entry, instance, raw_display_handle, raw_window_handle, None)
        } {
            Ok(surface) => surface,
            Err(e) => return Err("Failed to create surface: ".to_owned() + &e.to_string()),
        };

        let surface_loader = ash::khr::surface::Instance::new(entry, instance);

        let capabilities = match unsafe {
            surface_loader.get_physical_device_surface_capabilities(*physical_device, surface)
        } {
            Ok(capabilities) => capabilities,
            Err(e) => {
                return Err("Failed to query for surface capabilities: ".to_owned() + &e.to_string())
            }
        };

        let formats = match unsafe {
            surface_loader.get_physical_device_surface_formats(*physical_device, surface)
        } {
            Ok(formats) => formats,
            Err(e) => {
                return Err("Failed to query for surface formats: ".to_owned() + &e.to_string())
            }
        };

        Ok(Surface {
            surface,
            surface_loader,
            capabilities,
            formats,
        })
    }

    pub fn is_queue_family_supported(
        &self,
        physical_device: &PhysicalDevice,
        queue_family_index: u32,
    ) -> Result<bool, String> {
        match unsafe {
            self.surface_loader.get_physical_device_surface_support(
                *physical_device,
                queue_family_index,
                self.surface,
            )
        } {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(e) => Err("Failed to query for surface support: ".to_owned() + &e.to_string()),
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}
