use ash::vk::{self, PhysicalDevice};
use log::{trace, warn};

pub fn init_physical_device(instance: &ash::Instance) -> Result<PhysicalDevice, String> {
    trace!("Initializing: Vk Physical Device");

    let physical_devices = match unsafe { instance.enumerate_physical_devices() } {
        Ok(physical_devices) => physical_devices,
        Err(e) => {
            return Err("Renderer: Failed to enumerate physical devices: ".to_owned() + &e.to_string())
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
        panic!("Renderer: No physical devices found that meet the minimum requirements")
    }

    // Prefer a discrete GPU if available
    for p_device in meets_requirements_devices {
        let properties = unsafe { instance.get_physical_device_properties(*p_device) };

        if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
            return Ok(*p_device);
        }
    }

    warn!("Renderer: Unable to find discrete GPU, using first available that meets requirements");

    Ok(physical_devices[0])
}