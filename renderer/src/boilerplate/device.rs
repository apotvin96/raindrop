use ash::{
    vk::{self, PhysicalDevice},
    Device, Instance,
};
use log::trace;

pub fn init_device(
    instance: &Instance,
    physical_device: &PhysicalDevice,
    queue_indices: &[u32; 2],
) -> Result<Device, String> {
    trace!("Initializing: Vk Device");

    let mut extension_name_pointers: Vec<*const i8> = vec![ash::khr::swapchain::NAME.as_ptr()];

    let portability_extension = std::ffi::CString::new("VK_KHR_portability_subset").unwrap();

    if std::env::consts::OS == "macos" {
        extension_name_pointers.push(portability_extension.as_ptr());
    }

    let mut queue_create_infos = vec![vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_indices[0])
        .queue_priorities(&[1.0])];

    if queue_indices[0] != queue_indices[1] {
        queue_create_infos.push(
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_indices[1])
                .queue_priorities(&[1.0]),
        );
    }

    let device_create_info = vk::DeviceCreateInfo::default()
        .enabled_extension_names(&extension_name_pointers)
        .queue_create_infos(&queue_create_infos);

    let device =
        match unsafe { instance.create_device(*physical_device, &device_create_info, None) } {
            Ok(device) => device,
            Err(e) => return Err("Renderer: Failed to create device: ".to_owned() + &e.to_string()),
        };

    Ok(device)
}
