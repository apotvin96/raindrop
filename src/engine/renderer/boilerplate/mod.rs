use ash::{
    extensions::ext::DebugUtils,
    vk::{self, DebugUtilsMessengerEXT, InstanceCreateFlags, PhysicalDevice},
    Device, Entry, Instance,
};
use log::{trace, warn};

use raw_window_handle::HasRawDisplayHandle;

use std::{mem::ManuallyDrop, ops::BitOrAssign};

use vk_mem::{Allocator, AllocatorCreateInfo};

use crate::{config::Config, engine::renderer::debug};

use super::primitives::{CommandManager, Queue, Surface, Swapchain};

pub struct Boilerplate {
    pub instance: Instance,
    pub debug_messenger: DebugUtilsMessengerEXT,
    pub debug_loader: DebugUtils,
    pub physical_device: PhysicalDevice,
    pub surface: ManuallyDrop<Surface>,
    pub device: Device,
    pub allocator: ManuallyDrop<Allocator>,
    pub queue: Queue,
    pub swapchain: Swapchain,
    pub command_manager: ManuallyDrop<CommandManager>,
}

impl Boilerplate {
    pub fn new(config: &Config, window: &winit::window::Window) -> Result<Boilerplate, String> {
        let entry = Entry::linked();

        let (instance, debug_loader, debug_messenger) = match Self::init_instance(&entry, window) {
            Ok(info) => info,
            Err(e) => return Err("Failed to init renderer: instance: ".to_owned() + &e),
        };

        let physical_device = match Self::init_physical_device(&instance) {
            Ok(physical_device) => physical_device,
            Err(e) => return Err("Failed to init renderer: physical_device: ".to_owned() + &e),
        };

        let surface = match Surface::new(&entry, &instance, &physical_device, window) {
            Ok(surface) => surface,
            Err(e) => return Err("Failed to init renderer: surface: ".to_owned() + &e),
        };

        let queue_indices = match Queue::get_queue_indicies(&instance, &physical_device, &surface) {
            Ok(indices) => indices,
            Err(e) => return Err("Failed to init renderer: queue_indices: ".to_owned() + &e),
        };

        let device = match Self::init_device(&instance, &physical_device, &queue_indices) {
            Ok(device) => device,
            Err(e) => return Err("Failed to init renderer: device: ".to_owned() + &e),
        };

        let allocator = match Self::init_allocator(&instance, &physical_device, &device) {
            Ok(allocator) => allocator,
            Err(e) => return Err("Failed to init renderer: allocator: ".to_owned() + &e),
        };

        let queue = match Queue::new(&device, queue_indices[0], queue_indices[1]) {
            Ok(queue) => queue,
            Err(e) => return Err("Failed to init renderer: queue: ".to_owned() + &e),
        };

        let swapchain = match Swapchain::new(&instance, &device, &allocator, &surface, &queue) {
            Ok(swapchain) => swapchain,
            Err(e) => return Err("Failed to init renderer: swapchain: ".to_owned() + &e),
        };

        let command_manager = match CommandManager::new(&device, &queue) {
            Ok(command_manager) => command_manager,
            Err(e) => return Err("Failed to init renderer: command_manager: ".to_owned() + &e),
        };

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

    fn init_instance(
        entry: &ash::Entry,
        window: &winit::window::Window,
    ) -> Result<(ash::Instance, DebugUtils, DebugUtilsMessengerEXT), String> {
        trace!("Initializing Vk Instance");

        let engine_name = std::ffi::CString::new("Vulkan").unwrap();
        let application_name = std::ffi::CString::new("VKGuide App").unwrap();

        let application_info = vk::ApplicationInfo::builder()
            .api_version(vk::make_api_version(0, 1, 2, 0))
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
            extension_name_pointers.push(*extension);
        }

        let mut debug_create_info = debug::debug_create_info();

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

                vk::api_version_major(properties.api_version) >= 1
                    && vk::api_version_minor(properties.api_version) >= 2
            })
            .collect::<Vec<&PhysicalDevice>>();

        // There is nothing we can do if we don't have a physical device that meets the minimum requirements
        if meets_requirements_devices.is_empty() {
            panic!("No physical devices found that meet the minimum requirements")
        }

        // Prefer a discrete GPU if available
        for p_device in meets_requirements_devices {
            let properties = unsafe { instance.get_physical_device_properties(*p_device) };

            if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                return Ok(*p_device);
            }
        }

        warn!("Unable to find discrete GPU, using first available that meets requirements");

        Ok(physical_devices[0])
    }

    fn init_device(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        queue_indices: &[u32; 2],
    ) -> Result<Device, String> {
        trace!("Initializing: Vk Device");

        let mut extension_name_pointers: Vec<*const i8> =
            vec![ash::extensions::khr::Swapchain::name().as_ptr()];

        let portability_extension = std::ffi::CString::new("VK_KHR_portability_subset").unwrap();

        if std::env::consts::OS == "macos" {
            extension_name_pointers.push(portability_extension.as_ptr());
        }

        let mut queue_create_infos = vec![vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_indices[0])
            .queue_priorities(&[1.0])
            .build()];

        if queue_indices[0] != queue_indices[1] {
            queue_create_infos.push(
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(queue_indices[1])
                    .queue_priorities(&[1.0])
                    .build(),
            );
        }

        let device_create_info = vk::DeviceCreateInfo::builder()
            .enabled_extension_names(&extension_name_pointers)
            .queue_create_infos(&queue_create_infos)
            .build();

        let device =
            match unsafe { instance.create_device(*physical_device, &device_create_info, None) } {
                Ok(device) => device,
                Err(e) => return Err("Failed to create device: ".to_owned() + &e.to_string()),
            };

        Ok(device)
    }

    fn init_allocator(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        device: &Device,
    ) -> Result<Allocator, String> {
        trace!("Initializing: Vk Allocator");

        match Allocator::new(AllocatorCreateInfo::new(instance, device, *physical_device)) {
            Ok(allocator) => Ok(allocator),
            Err(e) => return Err("Failed to create allocator:".to_owned() + &e.to_string()),
        }
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
