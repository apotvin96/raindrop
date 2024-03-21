use std::mem::size_of;

use ash::{
    vk::{
        self, ColorComponentFlags, CullModeFlags, Extent2D, FrontFace, GraphicsPipelineCreateInfo,
        LogicOp, Offset2D, PipelineCache, PipelineColorBlendAttachmentState,
        PipelineColorBlendStateCreateInfo, PipelineDepthStencilStateCreateInfo,
        PipelineInputAssemblyStateCreateInfo, PipelineLayoutCreateFlags,
        PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
        PipelineShaderStageCreateInfo, PipelineVertexInputStateCreateInfo,
        PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, Rect2D, RenderPass,
        SampleCountFlags, Viewport,
    },
    Device,
};

use crate::engine::renderer::mesh::{MeshPushConstants, VertexInputDescription};

use super::Shader;

#[derive(Clone)]
pub struct Pipeline {
    device: Device,
    // TODO: Eventually we should have a pipeline cache that reuses pipeline layouts if they already exist
    pub pipeline_layout: ash::vk::PipelineLayout,
    pub pipeline: ash::vk::Pipeline,
}

impl Pipeline {
    pub fn new(
        device: &Device,
        shaders: &[&Shader],
        render_pass: &RenderPass,
        width: u32,
        height: u32,
        vertex_input_description: &VertexInputDescription,
    ) -> Result<Pipeline, String> {
        let push_constant_ranges = [ash::vk::PushConstantRange::builder()
            .stage_flags(ash::vk::ShaderStageFlags::VERTEX)
            .offset(0)
            .size(size_of::<MeshPushConstants>() as u32)
            .build()];

        let pipeline_layout_create_info = ash::vk::PipelineLayoutCreateInfo::builder()
            .flags(PipelineLayoutCreateFlags::empty())
            .push_constant_ranges(&push_constant_ranges)
            .set_layouts(&[])
            .build();

        let pipeline_layout =
            match unsafe { device.create_pipeline_layout(&pipeline_layout_create_info, None) } {
                Ok(pipeline_layout) => pipeline_layout,
                Err(err) => {
                    return Err(format!("Failed to create pipeline layout: {}", err));
                }
            };

        let color_blend_attachment_states = [PipelineColorBlendAttachmentState::builder()
            .blend_enable(false)
            .color_write_mask(ColorComponentFlags::RGBA)
            .build()];

        let color_blend_state = PipelineColorBlendStateCreateInfo::builder()
            .attachments(&color_blend_attachment_states)
            .logic_op_enable(false)
            .logic_op(LogicOp::COPY)
            .build();

        let input_assembly_state_create_info = PipelineInputAssemblyStateCreateInfo::builder()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();

        let multisample_state_create_info = PipelineMultisampleStateCreateInfo::builder()
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false)
            .sample_shading_enable(false)
            .rasterization_samples(SampleCountFlags::TYPE_1)
            .min_sample_shading(1.0)
            .build();

        let rasterization_state_create_info = PipelineRasterizationStateCreateInfo::builder()
            .cull_mode(CullModeFlags::NONE)
            .depth_clamp_enable(false)
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0)
            .front_face(FrontFace::CLOCKWISE)
            .line_width(1.0)
            .polygon_mode(PolygonMode::FILL)
            .rasterizer_discard_enable(false)
            .build();

        let vertex_input_state_create_info = PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&vertex_input_description.attribute_descriptions)
            .vertex_binding_descriptions(&vertex_input_description.binding_descriptions)
            .build();

        let viewports = [Viewport {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        let scissors = [Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: Extent2D { width, height },
        }];

        let viewport_state_create_info = PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors)
            .build();

        let shader_stage_create_infos = shaders
            .iter()
            .map(|shader| shader.stage_create_info())
            .collect::<Vec<PipelineShaderStageCreateInfo>>();

        let depth_stencil_state = PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::LESS_OR_EQUAL)
            .depth_bounds_test_enable(false)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0)
            .stencil_test_enable(false)
            .build();

        let pipeline_create_info = GraphicsPipelineCreateInfo::builder()
            .color_blend_state(&color_blend_state)
            .depth_stencil_state(&depth_stencil_state)
            .input_assembly_state(&input_assembly_state_create_info)
            .layout(pipeline_layout)
            .multisample_state(&multisample_state_create_info)
            .rasterization_state(&rasterization_state_create_info)
            .render_pass(*render_pass)
            .stages(&shader_stage_create_infos)
            .subpass(0)
            .vertex_input_state(&vertex_input_state_create_info)
            .viewport_state(&viewport_state_create_info)
            .build();

        let pipeline = match unsafe {
            device.create_graphics_pipelines(PipelineCache::null(), &[pipeline_create_info], None)
        } {
            Ok(pipelines) => pipelines[0],
            Err(err) => {
                return Err(format!("Failed to create pipeline: {}", err.1));
            }
        };

        Ok(Pipeline {
            device: device.clone(),
            pipeline_layout,
            pipeline,
        })
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_pipeline(self.pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}
