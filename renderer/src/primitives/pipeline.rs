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

use crate::mesh::{MeshPushConstants, VertexInputDescription};

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
        let push_constant_ranges = [ash::vk::PushConstantRange::default()
            .stage_flags(ash::vk::ShaderStageFlags::VERTEX)
            .offset(0)
            .size(size_of::<MeshPushConstants>() as u32)];

        let pipeline_layout_create_info = ash::vk::PipelineLayoutCreateInfo::default()
            .flags(PipelineLayoutCreateFlags::empty())
            .push_constant_ranges(&push_constant_ranges)
            .set_layouts(&[]);

        let pipeline_layout =
            match unsafe { device.create_pipeline_layout(&pipeline_layout_create_info, None) } {
                Ok(pipeline_layout) => pipeline_layout,
                Err(err) => {
                    return Err(format!("Failed to create pipeline layout: {}", err));
                }
            };

        let color_blend_attachment_states = [PipelineColorBlendAttachmentState::default()
            .blend_enable(false)
            .color_write_mask(ColorComponentFlags::RGBA)];

        let color_blend_state = PipelineColorBlendStateCreateInfo::default()
            .attachments(&color_blend_attachment_states)
            .logic_op_enable(false)
            .logic_op(LogicOp::COPY);

        let input_assembly_state_create_info = PipelineInputAssemblyStateCreateInfo::default()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let multisample_state_create_info = PipelineMultisampleStateCreateInfo::default()
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false)
            .sample_shading_enable(false)
            .rasterization_samples(SampleCountFlags::TYPE_1)
            .min_sample_shading(1.0);

        let rasterization_state_create_info = PipelineRasterizationStateCreateInfo::default()
            .cull_mode(CullModeFlags::NONE)
            .depth_clamp_enable(false)
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0)
            .front_face(FrontFace::CLOCKWISE)
            .line_width(1.0)
            .polygon_mode(PolygonMode::FILL)
            .rasterizer_discard_enable(false);

        let vertex_input_state_create_info = PipelineVertexInputStateCreateInfo::default()
            .vertex_attribute_descriptions(&vertex_input_description.attribute_descriptions)
            .vertex_binding_descriptions(&vertex_input_description.binding_descriptions);

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

        let viewport_state_create_info = PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);

        let shader_stage_create_infos = shaders
            .iter()
            .map(|shader| shader.stage_create_info())
            .collect::<Vec<PipelineShaderStageCreateInfo>>();

        let depth_stencil_state = PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::LESS_OR_EQUAL)
            .depth_bounds_test_enable(false)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0)
            .stencil_test_enable(false);

        let pipeline_create_info = GraphicsPipelineCreateInfo::default()
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
            .viewport_state(&viewport_state_create_info);

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
