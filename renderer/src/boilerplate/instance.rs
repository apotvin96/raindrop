use ash::{
    extensions::ext::DebugUtils,
    vk::{self, DebugUtilsMessengerEXT, InstanceCreateFlags},
};
use log::trace;

use raw_window_handle::HasRawDisplayHandle;

use std::ops::BitOrAssign;

use crate::debug;

pub fn init_instance(
    entry: &ash::Entry,
    window: &winit::window::Window,
) -> Result<(ash::Instance, DebugUtils, DebugUtilsMessengerEXT), String> {
    trace!("Initializing Vk Instance");

    let engine_name = std::ffi::CString::new("Vulkan").unwrap();
    let application_name = std::ffi::CString::new("Raindrop App").unwrap();

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
        Err(e) => return Err("Renderer: Failed to create debug messenger: ".to_owned() + &e.to_string()),
    };

    Ok((instance, debug_utils_loader, debug_utils_messenger))
}
