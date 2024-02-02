use ash::{vk::SurfaceKHR, Entry, Instance};
use winit::window::Window;

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub struct Surface {
    surface: SurfaceKHR,
    surface_loader: ash::extensions::khr::Surface,
}

impl Surface {
    pub fn new(entry: &Entry, instance: &Instance, window: &Window) -> Result<Surface, String> {
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

        Ok(Surface {
            surface,
            surface_loader,
        })
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}
