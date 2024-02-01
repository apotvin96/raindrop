use std::ops::BitOrAssign;

use ash::{
    extensions::ext::DebugUtils,
    vk::{self, DebugUtilsMessengerEXT, InstanceCreateFlags, PhysicalDevice, SurfaceKHR},
    Entry,
};
use log::{error, info, trace, warn};

use crate::config::Config;

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();

    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            error!("Vk Validation Layer Error: {} {:?}", ty, message);
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            warn!("Vk Validation Layer Warn: {} {:?}", ty, message);
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            info!("Vk Validation Layer Info: {} {:?}", ty, message);
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
            trace!("Vk Validation Layer Trace: {} {:?}", ty, message);
        }
        _ => {
            error!("Vk Validation Layer Unknown: {} {:?}", ty, message)
        }
    }

    vk::FALSE
}

pub struct Renderer {
    entry: ash::Entry,
    instance: ash::Instance,
    debug_messenger: DebugUtilsMessengerEXT,
    debug_loader: DebugUtils,
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    surface_loader: ash::extensions::khr::Surface
}

impl Renderer {
    pub fn new(config: Config, window: &winit::window::Window) -> Result<Renderer, String> {
        trace!("Initializing: Renderer");

        let entry = Entry::linked();

        let (instance, debug_loader, debug_messenger) = match Self::init_instance(&entry, window) {
            Ok(info) => info,
            Err(e) => return Err("Failed to init renderer: instance: ".to_owned() + &e),
        };

        let physical_device = match Self::init_physical_device(&instance) {
            Ok(physical_device) => physical_device,
            Err(e) => return Err("Failed to init renderer: physical_device: ".to_owned() + &e),
        };

        let (surface, surface_loader)= match Self::init_surface(&entry, &window, &instance, physical_device) {
            Ok(surface) => surface,
            Err(e) => return Err("Failed to init renderer: surface: ".to_owned() + &e),
        };

        Ok(Renderer {
            entry,
            instance,
            debug_loader,
            debug_messenger,
            physical_device,
            surface,
            surface_loader
        })
    }

    fn init_instance(
        entry: &ash::Entry,
        window: &winit::window::Window,
    ) -> Result<(ash::Instance, DebugUtils, DebugUtilsMessengerEXT), String> {
        trace!("Initializing Vk Instance");

        let engine_name = std::ffi::CString::new("Vulkan").unwrap();
        let application_name = std::ffi::CString::new("VKGuide App").unwrap();

        let application_info = vk::ApplicationInfo::builder()
            .api_version(vk::make_api_version(0, 1, 3, 0))
            .engine_version(vk::make_api_version(0, 0, 0, 1))
            .engine_name(&engine_name)
            .application_version(vk::make_api_version(0, 0, 0, 1))
            .application_name(&application_name);

        let layer_names: Vec<std::ffi::CString> =
            vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        let enabled_layer_names: Vec<*const i8> = layer_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let mut extension_name_pointers: Vec<*const i8> = vec![
            ash::extensions::ext::DebugUtils::name().as_ptr(),
            ash::extensions::khr::Surface::name().as_ptr(),
        ];
        let mut instance_create_flags = InstanceCreateFlags::empty();

        let portability_extension =
            std::ffi::CString::new("VK_KHR_portability_enumeration").unwrap();
        let physical_device_properties2_extension =
            std::ffi::CString::new("VK_KHR_get_physical_device_properties2").unwrap();
        if std::env::consts::OS == "macos" {
            extension_name_pointers.push(portability_extension.as_ptr());
            extension_name_pointers.push(physical_device_properties2_extension.as_ptr());
            instance_create_flags.bitor_assign(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR);
        }

        let surface_extensions =
            ash_window::enumerate_required_extensions(window.raw_display_handle()).unwrap();

        for extension in surface_extensions {
            extension_name_pointers.push(extension.clone());
        }

        let mut debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            )
            .pfn_user_callback(Some(vulkan_debug_utils_callback));

        let instance_create_info = vk::InstanceCreateInfo::builder()
            .push_next(&mut debug_create_info)
            .application_info(&application_info)
            .enabled_layer_names(&enabled_layer_names)
            .enabled_extension_names(&extension_name_pointers)
            .flags(instance_create_flags);

        let instance = match unsafe { entry.create_instance(&instance_create_info, None) } {
            Ok(instance) => instance,
            Err(e) => return Err("Failed to create instance: ".to_owned() + &e.to_string()),
        };

        let debug_utils_loader = ash::extensions::ext::DebugUtils::new(entry, &instance);
        let debug_utils_messenger = match unsafe {
            debug_utils_loader.create_debug_utils_messenger(&debug_create_info, None)
        } {
            Ok(messenger) => messenger,
            Err(e) => return Err("Failed to create debug messenger: ".to_owned() + &e.to_string()),
        };

        Ok((instance, debug_utils_loader, debug_utils_messenger))
    }

    fn init_physical_device(instance: &ash::Instance) -> Result<PhysicalDevice, String> {
        trace!("Initializing: Vk Physical Device");

        let physical_devices = match unsafe { instance.enumerate_physical_devices() } {
            Ok(physical_devices) => physical_devices,
            Err(e) => {
                return Err("Failed to enumerate physical devices: ".to_owned() + &e.to_string())
            }
        };

        // Find the absolute minimum requirements for a physical device
        let meets_requirements_devices = physical_devices
            .iter()
            .filter(|&p_device| {
                let properties = unsafe { instance.get_physical_device_properties(*p_device) };

                return vk::api_version_major(properties.api_version) >= 1
                    && vk::api_version_minor(properties.api_version) >= 2;
            })
            .collect::<Vec<&PhysicalDevice>>();

        // There is nothing we can do if we don't have a physical device that meets the requirements
        if meets_requirements_devices.len() == 0 {
            panic!("No physical devices found that meet the requirements")
        }

        // Prefer a discrete GPU if available
        for p_device in meets_requirements_devices {
            let properties = unsafe { instance.get_physical_device_properties(*p_device) };

            if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                return Ok(*p_device);
            }
        }

        warn!("Unable to find discrete GPU with requested properties, using first available");

        return Ok(physical_devices[0]);
    }

    fn init_surface(
        entry: &ash::Entry,
        window: &winit::window::Window,
        instance: &ash::Instance,
        physical_device: PhysicalDevice,
    ) -> Result<(SurfaceKHR, ash::extensions::khr::Surface), String> {
        trace!("Initializing: Vk Surface");

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

        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

        Ok((surface, surface_loader))
    }

    pub fn render(&mut self) {
        trace!("Rendering");
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        trace!("Cleaning: Renderer");

        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
            self.debug_loader
                .destroy_debug_utils_messenger(self.debug_messenger, None);
            self.instance.destroy_instance(None);
        }
    }
}
