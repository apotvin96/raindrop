use ash::{
    vk::{PhysicalDevice, SurfaceKHR},
    Entry, Instance,
};
use winit::window::Window;

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub struct Surface {
    pub surface: SurfaceKHR,
    surface_loader: ash::extensions::khr::Surface,
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
        let surface = match unsafe {
            ash_window::create_surface(
                entry,
                instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
        } {
            Ok(surface) => surface,
            Err(e) => return Err("Failed to create surface: ".to_owned() + &e.to_string()),
        };

        let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

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
