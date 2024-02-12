use ash::vk;
use log::{error, info, trace, warn};

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
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

pub fn debug_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        )
        .pfn_user_callback(Some(vulkan_debug_utils_callback))
        .build()
}
