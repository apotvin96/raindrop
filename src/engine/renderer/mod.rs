mod debug;
mod mesh;
mod primitives;

use std::{mem::ManuallyDrop, ops::BitOrAssign};

use ash::{
    extensions::ext::DebugUtils,
    vk::{
        self, AttachmentDescription, AttachmentLoadOp, AttachmentStoreOp, ClearValue,
        DebugUtilsMessengerEXT, Framebuffer, FramebufferCreateInfo, ImageLayout,
        InstanceCreateFlags, PhysicalDevice, Rect2D, RenderPass, RenderPassCreateInfo,
        SampleCountFlags, SubpassDescription,
    },
    Device, Entry, Instance,
};
use log::{trace, warn};

use raw_window_handle::HasRawDisplayHandle;
use vk_mem::{Allocator, AllocatorCreateInfo};

use crate::{config::Config, engine::renderer::mesh::MeshPushConstants};

use primitives::CommandManager;
use primitives::Pipeline;
use primitives::Queue;
use primitives::Shader;
use primitives::Surface;
use primitives::Swapchain;

use mesh::Mesh;
use mesh::Vertex;

pub struct Renderer {
    framenumber: u64,
    instance: Instance,
    debug_messenger: DebugUtilsMessengerEXT,
    debug_loader: DebugUtils,
    physical_device: PhysicalDevice,
    surface: ManuallyDrop<Surface>,
    device: Device,
    allocator: ManuallyDrop<Allocator>,
    queue: Queue,
    swapchain: ManuallyDrop<Swapchain>,
    command_manager: ManuallyDrop<CommandManager>,
    render_pass: RenderPass,
    framebuffers: Vec<Framebuffer>,
    render_semaphore: vk::Semaphore,
    present_semaphore: vk::Semaphore,
    fence: vk::Fence,
    mesh_pipeline: ManuallyDrop<Pipeline>,
    mesh: Mesh,
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

        let mut allocator = match Self::init_allocator(&instance, &physical_device, &device) {
            Ok(allocator) => allocator,
            Err(e) => return Err("Failed to init renderer: allocator: ".to_owned() + &e),
        };

        let queue = match Queue::new(&device, queue_indices[0], queue_indices[1]) {
            Ok(queue) => queue,
            Err(e) => return Err("Failed to init renderer: queue: ".to_owned() + &e),
        };

        let swapchain = match Swapchain::new(&instance, &device, &surface, &queue) {
            Ok(swapchain) => swapchain,
            Err(e) => return Err("Failed to init renderer: swapchain: ".to_owned() + &e),
        };

        let command_manager = match CommandManager::new(&device, &queue) {
            Ok(command_manager) => command_manager,
            Err(e) => return Err("Failed to init renderer: command_manager: ".to_owned() + &e),
        };

        let render_pass = match Self::init_render_pass(&device, &swapchain) {
            Ok(render_pass) => render_pass,
            Err(e) => return Err("Failed to init renderer: render_pass: ".to_owned() + &e),
        };

        let framebuffers = match Self::init_frame_buffers(&device, &swapchain, &render_pass) {
            Ok(framebuffers) => framebuffers,
            Err(e) => return Err("Failed to init renderer: framebuffers: ".to_owned() + &e),
        };

        let vertex_shader = match Shader::from_path(&device, "assets/shaders/tri_mesh.vert") {
            Ok(shader) => shader,
            Err(e) => return Err("Failed to create vertex shader: ".to_owned() + &e.to_string()),
        };

        let color_fragment_shader =
            match Shader::from_path(&device, "assets/shaders/colored_triangle.frag") {
                Ok(shader) => shader,
                Err(e) => {
                    return Err("Failed to create fragment shader: ".to_owned() + &e.to_string())
                }
            };

        let mesh_pipeline = match Pipeline::new(
            &device,
            &[&vertex_shader, &color_fragment_shader],
            &render_pass,
            swapchain.extent.width,
            swapchain.extent.height,
            &Vertex::get_vertex_input_description(),
        ) {
            Ok(pipeline) => pipeline,
            Err(e) => return Err("Failed to create pipeline: ".to_owned() + &e.to_string()),
        };

        let fence_create_info =
            vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

        let fence = match unsafe { device.create_fence(&fence_create_info, None) } {
            Ok(fence) => fence,
            Err(e) => return Err("Failed to create fence: ".to_owned() + &e.to_string()),
        };

        let semaphore_create_info = vk::SemaphoreCreateInfo::builder().build();

        let render_semaphore =
            match unsafe { device.create_semaphore(&semaphore_create_info, None) } {
                Ok(semaphore) => semaphore,
                Err(e) => return Err("Failed to create semaphore: ".to_owned() + &e.to_string()),
            };

        let present_semaphore =
            match unsafe { device.create_semaphore(&semaphore_create_info, None) } {
                Ok(semaphore) => semaphore,
                Err(e) => return Err("Failed to create semaphore: ".to_owned() + &e.to_string()),
            };

        let mesh = Self::init_mesh(&mut allocator);

        Ok(Renderer {
            framenumber: 0,
            instance,
            debug_loader,
            debug_messenger,
            physical_device,
            surface: ManuallyDrop::new(surface),
            device,
            allocator: ManuallyDrop::new(allocator),
            queue,
            swapchain: ManuallyDrop::new(swapchain),
            command_manager: ManuallyDrop::new(command_manager),
            render_pass,
            framebuffers,
            fence,
            render_semaphore,
            present_semaphore,
            mesh_pipeline: ManuallyDrop::new(mesh_pipeline),
            mesh,
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

        // match Allocator::new(&AllocatorCreateDesc {
        //     instance: instance.clone(),
        //     device: device.clone(),
        //     physical_device: physical_device.clone(),
        //     debug_settings: Default::default(),
        //     buffer_device_address: false, // TODO: Investigate whether I want to enable this extension on the device later
        //     allocation_sizes: Default::default(),
        // }) {
        //     Ok(allocator) => Ok(allocator),
        //     Err(e) => return Err("Failed to create allocator:".to_owned() + &e.to_string()),
        // }

        match Allocator::new(AllocatorCreateInfo::new(instance, device, *physical_device)) {
            Ok(allocator) => Ok(allocator),
            Err(e) => return Err("Failed to create allocator:".to_owned() + &e.to_string()),
        }
    }

    fn init_render_pass(device: &Device, swapchain: &Swapchain) -> Result<RenderPass, String> {
        trace!("Initializing: Vk RenderPass");

        let attachment_description = AttachmentDescription::builder()
            .format(swapchain.image_format)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR)
            .build();

        let attachment_references = [vk::AttachmentReference::builder()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build()];

        let subpass_descriptions = [SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&attachment_references)
            .build()];

        let render_pass_create_info = RenderPassCreateInfo::builder()
            .attachments(&[attachment_description])
            .subpasses(&subpass_descriptions)
            .build();

        match unsafe { device.create_render_pass(&render_pass_create_info, None) } {
            Ok(render_pass) => Ok(render_pass),
            Err(e) => Err("Failed to create render pass: ".to_owned() + &e.to_string()),
        }
    }

    fn init_frame_buffers(
        device: &Device,
        swapchain: &Swapchain,
        render_pass: &RenderPass,
    ) -> Result<Vec<Framebuffer>, String> {
        trace!("Initializing: Vk Framebuffers");

        let mut framebuffers: Vec<Framebuffer> = Vec::with_capacity(swapchain.image_views.len());

        for image_view in &swapchain.image_views {
            let attachments = [*image_view];

            let framebuffer_create_info = FramebufferCreateInfo::builder()
                .render_pass(*render_pass)
                .width(swapchain.extent.width)
                .height(swapchain.extent.height)
                .layers(1)
                .attachments(&attachments)
                .build();

            let framebuffer = match unsafe {
                device.create_framebuffer(&framebuffer_create_info, None)
            } {
                Ok(framebuffer) => framebuffer,
                Err(e) => return Err("Failed to create framebuffer: ".to_owned() + &e.to_string()),
            };

            framebuffers.push(framebuffer);
        }

        Ok(framebuffers)
    }

    fn init_mesh(allocator: &mut Allocator) -> Mesh {
        trace!("Initializing: Mesh");

        let mut mesh = Mesh::from_path("assets/models/monkey/monkey.glb");

        mesh.upload(allocator).unwrap();

        mesh
    }

    pub fn render(&mut self) {
        trace!("Rendering");

        unsafe { self.device.wait_for_fences(&[self.fence], true, 1000000000) }
            .expect("Failed to wait for fence");

        unsafe { self.device.reset_fences(&[self.fence]) }.expect("Failed to reset fence");

        let (image_index, _) = self
            .swapchain
            .acquire_next_image(self.present_semaphore)
            .expect("Failed to acquire next image");

        self.command_manager.begin_main_command_buffer();

        //let flash = (self.framenumber as f32 / 120.0).sin().abs();
        let flash = 0.0;

        let clear_value = ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, flash, 1.0],
            },
        };

        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass)
            .render_area(Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain.extent,
            })
            .framebuffer(self.framebuffers[image_index as usize])
            .clear_values(&[clear_value])
            .build();

        self.command_manager
            .begin_render_pass(&render_pass_begin_info);

        self.command_manager.bind_pipeline(&self.mesh_pipeline);

        let offset = 0;
        self.command_manager.bind_vertex_buffers(
            0,
            &[self.mesh.vertex_buffer.as_mut().unwrap().buffer],
            &[offset],
        );

        let cam_pos = glm::vec3(0.0, 0.0, -2.0);
        let view_mat = glm::translate(&glm::Mat4::identity(), &cam_pos);

        let mut proj_mat = glm::perspective(
            1600.0 / 900.0,
            glm::radians(&glm::vec1(70.0))[0],
            0.1,
            200.0,
        );

        proj_mat[(1, 1)] *= -1.0;

        let model_mat = glm::rotate(
            &glm::Mat4::identity(),
            self.framenumber as f32 / 100.0,
            &glm::vec3(0.0, 1.0, 0.0),
        );

        let matrix = proj_mat * view_mat * model_mat;

        let push_constants = MeshPushConstants {
            render_matrix: matrix,
            data: glm::vec4(0.0, 0.0, 0.0, 0.0),
        };

        self.command_manager
            .push_constants(self.mesh_pipeline.pipeline_layout, push_constants);

        self.command_manager
            .draw(self.mesh.vertices.len() as u32, 1, 0, 0);

        self.command_manager.end_render_pass();

        self.command_manager.end_main_command_buffer().unwrap();

        self.command_manager.submit_main_command_buffer(
            &[self.present_semaphore],
            &[self.present_semaphore],
            self.fence,
        );

        self.swapchain
            .present(&self.queue, image_index, &[self.present_semaphore]);

        self.framenumber += 1;
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        trace!("Cleaning: Renderer");

        unsafe {
            self.device
                .wait_for_fences(&[self.fence], true, 1000000000)
                .expect("Failed to wait for fence");

            self.mesh.free(&mut self.allocator);

            ManuallyDrop::drop(&mut self.allocator);

            self.device.destroy_semaphore(self.render_semaphore, None);
            self.device.destroy_semaphore(self.present_semaphore, None);
            self.device.destroy_fence(self.fence, None);

            ManuallyDrop::drop(&mut self.mesh_pipeline);

            for framebuffer in &self.framebuffers {
                self.device.destroy_framebuffer(*framebuffer, None);
            }
            self.device.destroy_render_pass(self.render_pass, None);
            ManuallyDrop::drop(&mut self.command_manager);
            ManuallyDrop::drop(&mut self.swapchain);
            self.device.destroy_device(None);
            ManuallyDrop::drop(&mut self.surface);
            self.debug_loader
                .destroy_debug_utils_messenger(self.debug_messenger, None);
            self.instance.destroy_instance(None);
        }
    }
}
