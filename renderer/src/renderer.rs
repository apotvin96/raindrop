use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ash::{
    vk::{
        self, AccessFlags, AttachmentDescription, AttachmentLoadOp, AttachmentStoreOp, ClearValue,
        Framebuffer, FramebufferCreateInfo, ImageLayout, PipelineStageFlags, Rect2D, RenderPass,
        RenderPassCreateInfo, SampleCountFlags, SubpassDependency, SubpassDescription,
        SUBPASS_EXTERNAL,
    },
    Device,
};
use asset_manager::AssetManager;
use log::trace;

use config::Config;

use crate::Boilerplate;
use crate::Material;
use crate::Renderable;
use crate::{boilerplate::frame_data::FrameData, mesh::MeshPushConstants};
use crate::{
    mesh::Vertex,
    primitives::{Pipeline, Shader, Swapchain},
};

pub struct Renderer {
    config: Config,
    boilerplate: Boilerplate,
    render_pass: RenderPass,
    framebuffers: Vec<Framebuffer>,
    pipelines: HashMap<String, Rc<RefCell<Pipeline>>>,
    materials: HashMap<String, Rc<RefCell<Material>>>,
    framenumber: u64,
    mesh_binds: u64,
    material_binds: u64,
}

impl Renderer {
    pub fn new(config: &Config, window: &winit::window::Window) -> Result<Renderer, String> {
        trace!("Initializing: Renderer");

        let boilerplate = match Boilerplate::new(config, window) {
            Ok(boilerplate) => boilerplate,
            Err(e) => return Err("Failed to init boilerplate: ".to_owned() + &e),
        };

        let render_pass = match Self::init_render_pass(&boilerplate.device, &boilerplate.swapchain)
        {
            Ok(render_pass) => render_pass,
            Err(e) => return Err("Failed to init renderer: render_pass: ".to_owned() + &e),
        };

        let framebuffers = match Self::init_frame_buffers(
            &boilerplate.device,
            &boilerplate.swapchain,
            &render_pass,
        ) {
            Ok(framebuffers) => framebuffers,
            Err(e) => return Err("Failed to init renderer: framebuffers: ".to_owned() + &e),
        };

        let vertex_shader =
            match Shader::from_path(&boilerplate.device, "assets/shaders/tri_mesh.vert") {
                Ok(shader) => shader,
                Err(e) => {
                    return Err("Failed to create vertex shader: ".to_owned() + &e.to_string())
                }
            };

        let color_fragment_shader =
            match Shader::from_path(&boilerplate.device, "assets/shaders/colored_triangle.frag") {
                Ok(shader) => shader,
                Err(e) => {
                    return Err("Failed to create fragment shader: ".to_owned() + &e.to_string())
                }
            };

        let mesh_pipeline = match Pipeline::new(
            &boilerplate.device,
            &[&vertex_shader, &color_fragment_shader],
            &render_pass,
            boilerplate.swapchain.extent.width,
            boilerplate.swapchain.extent.height,
            &Vertex::get_vertex_input_description(),
        ) {
            Ok(pipeline) => pipeline,
            Err(e) => return Err("Failed to create pipeline: ".to_owned() + &e.to_string()),
        };

        let mut pipelines = HashMap::new();
        pipelines.insert(
            "meshpipeline".to_string(),
            Rc::new(RefCell::new(mesh_pipeline)),
        );

        let mut materials = HashMap::new();
        materials.insert(
            "defaultmesh".to_string(),
            Rc::new(RefCell::new(Material {
                pipeline: Rc::clone(pipelines.get("meshpipeline").unwrap()),
            })),
        );

        Ok(Renderer {
            config: config.clone(),
            boilerplate,
            render_pass,
            framebuffers,
            pipelines,
            materials,
            framenumber: 0,
            mesh_binds: 0,
            material_binds: 0,
        })
    }

    fn init_render_pass(device: &Device, swapchain: &Swapchain) -> Result<RenderPass, String> {
        trace!("Initializing: Vk RenderPass");

        let attachment_description = AttachmentDescription::default()
            .format(swapchain.image_format)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR);

        let attachment_references = [vk::AttachmentReference::default()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        let depth_attachment_description = AttachmentDescription::default()
            .format(vk::Format::D32_SFLOAT)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::CLEAR)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        let depth_attachment_references = vk::AttachmentReference::default()
            .attachment(1)
            .layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        let subpass_descriptions = [SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&attachment_references)
            .depth_stencil_attachment(&depth_attachment_references)];

        let color_dependency = SubpassDependency::default()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(AccessFlags::NONE)
            .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE);

        let depth_dependency = SubpassDependency::default()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(
                PipelineStageFlags::EARLY_FRAGMENT_TESTS | PipelineStageFlags::LATE_FRAGMENT_TESTS,
            )
            .src_access_mask(AccessFlags::NONE)
            .dst_stage_mask(
                PipelineStageFlags::EARLY_FRAGMENT_TESTS | PipelineStageFlags::LATE_FRAGMENT_TESTS,
            )
            .dst_access_mask(AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE);

        let attachments = [attachment_description, depth_attachment_description];
        let dependencies = [color_dependency, depth_dependency];

        let render_pass_create_info = RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpass_descriptions)
            .dependencies(&dependencies);

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
            let attachments = [*image_view, swapchain.depth_image_view];

            let framebuffer_create_info = FramebufferCreateInfo::default()
                .render_pass(*render_pass)
                .width(swapchain.extent.width)
                .height(swapchain.extent.height)
                .layers(1)
                .attachments(&attachments);

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

    fn current_frame_data(&self) -> &FrameData {
        &self.boilerplate.frame_data
            [(self.framenumber % self.config.renderer.frame_overlap as u64) as usize]
    }

    fn bind_renderable_mesh(
        &mut self,
        renderable: &Renderable,
        asset_manager: &mut AssetManager,
    ) -> (bool, String, u32) {
        let mesh_handle = asset_manager.get_mesh(&renderable.mesh);
        let lock = mesh_handle.lock();
        let mut mesh = lock.unwrap();

        if mesh.needs_uploaded() {
            let vertices = mesh.vertices.clone();

            mesh.add_gpu_info(self.boilerplate.allocator.create_vertex_buffer(&vertices));
        }

        let mut can_be_drawn = false;

        if mesh.gpu_info.is_some() {
            let offset = 0;
            let buffer = mesh.gpu_info.as_mut().unwrap().buffer;

            self.current_frame_data()
                .command_manager
                .bind_vertex_buffers(0, &[buffer], &[offset]);

            can_be_drawn = true;
            self.mesh_binds += 1;
        }

        (can_be_drawn, renderable.mesh.clone(), mesh.vertex_count)
    }

    fn bind_renderable_material(
        &mut self,
        renderable: &Renderable,
    ) -> (Option<Rc<RefCell<Material>>>, String) {
        let material = self.materials.get(&renderable.material).unwrap();

        self.current_frame_data()
            .command_manager
            .bind_pipeline(&material.borrow().pipeline.borrow());

        let last_material = Some(Rc::clone(self.materials.get(&renderable.material).unwrap()));
        let last_material_id = renderable.material.clone();

        self.material_binds += 1;

        (last_material, last_material_id)
    }

    fn render_objects(
        &mut self,
        mut projection_matrix: glm::Mat4,
        view_matrix: glm::Mat4,
        renderables: &[Renderable],
        asset_manager: &mut AssetManager,
    ) {
        self.mesh_binds = 0;
        self.material_binds = 0;

        // Flip the y axis to match the Vulkan coordinate system
        projection_matrix[(1, 1)] *= -1.0;
        let view_proj_mat = projection_matrix * view_matrix;

        let mut last_mesh_id: String = "".to_string();
        let mut last_mesh_vertex_count = 0;

        let mut last_material: Option<Rc<RefCell<Material>>> = None;
        let mut last_material_id: String = "".to_string();

        for renderable in renderables {
            if renderable.mesh != last_mesh_id {
                let (can_be_drawn, last_bound_mesh_id, last_bound_mesh_vertex_count) =
                    self.bind_renderable_mesh(renderable, asset_manager);

                if !can_be_drawn {
                    continue;
                } else {
                    last_mesh_id = last_bound_mesh_id;
                    last_mesh_vertex_count = last_bound_mesh_vertex_count;
                }
            }

            if renderable.material != last_material_id {
                (last_material, last_material_id) = self.bind_renderable_material(renderable);
            }

            let mvp = view_proj_mat * renderable.matrix;

            let push_constants = MeshPushConstants {
                data: glm::vec4(0.0, 0.0, 0.0, 0.0),
                render_matrix: mvp,
            };

            self.current_frame_data().command_manager.push_constants(
                last_material
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .pipeline
                    .borrow()
                    .pipeline_layout,
                push_constants,
            );

            self.current_frame_data()
                .command_manager
                .draw(last_mesh_vertex_count, 1, 0, 0);
        }

        trace!(
            "> Rendered {} objects with {} mesh bind(s) and {} material bind(s)",
            renderables.len(),
            self.mesh_binds,
            self.material_binds
        );
    }

    pub fn render(
        &mut self,
        projection_matrix: glm::Mat4,
        view_matrix: glm::Mat4,
        renderables: &[Renderable],
        asset_manager: &mut AssetManager,
    ) {
        trace!("Renderer Rendering");

        unsafe {
            self.boilerplate.device.wait_for_fences(
                &[self.current_frame_data().render_fence],
                true,
                1000000000,
            )
        }
        .expect("Failed to wait for fence");

        unsafe {
            self.boilerplate
                .device
                .reset_fences(&[self.current_frame_data().render_fence])
        }
        .expect("Failed to reset fence");

        let (image_index, _) = self
            .boilerplate
            .swapchain
            .acquire_next_image(self.current_frame_data().present_semaphore)
            .expect("Failed to acquire next image");

        self.current_frame_data()
            .command_manager
            .begin_main_command_buffer();

        let flash = 0.0;

        let clear_values = [
            ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, flash, 1.0],
                },
            },
            ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ];

        let render_pass_begin_info = vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass)
            .render_area(Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.boilerplate.swapchain.extent,
            })
            .framebuffer(self.framebuffers[image_index as usize])
            .clear_values(&clear_values);

        self.current_frame_data()
            .command_manager
            .begin_render_pass(&render_pass_begin_info);

        self.render_objects(projection_matrix, view_matrix, renderables, asset_manager);

        self.current_frame_data().command_manager.end_render_pass();

        self.current_frame_data()
            .command_manager
            .end_main_command_buffer()
            .unwrap();

        self.current_frame_data()
            .command_manager
            .submit_main_command_buffer(
                &[self.current_frame_data().present_semaphore],
                &[self.current_frame_data().present_semaphore],
                self.current_frame_data().render_fence,
            );

        self.boilerplate.swapchain.present(
            &self.boilerplate.queue,
            image_index,
            &[self.current_frame_data().present_semaphore],
        );

        self.framenumber += 1;
    }

    pub fn cleanup(&mut self, asset_manager: &mut AssetManager) {
        trace!("Cleaning: Renderer");

        unsafe {
            self.boilerplate.wait_for_fences();

            self.materials = HashMap::new();
            self.pipelines = HashMap::new();

            for (_, mesh_clone) in asset_manager.iter_meshes_mut().lock().unwrap().iter_mut() {
                let mesh_handle = mesh_clone.lock();

                let mut mesh = mesh_handle.unwrap();

                if let Some(gpu_info) = &mut mesh.gpu_info {
                    self.boilerplate.allocator.destroy_buffer(gpu_info)
                };
            }

            for framebuffer in &self.framebuffers {
                self.boilerplate
                    .device
                    .destroy_framebuffer(*framebuffer, None);
            }

            self.boilerplate
                .device
                .destroy_render_pass(self.render_pass, None);
        }
    }
}
