use std::mem::size_of;

use ash::vk::{
    PipelineVertexInputStateCreateFlags, VertexInputAttributeDescription,
    VertexInputBindingDescription, VertexInputRate,
};

use memoffset::offset_of;
use serde_derive::Serialize;

pub struct VertexInputDescription {
    pub binding_descriptions: Vec<VertexInputBindingDescription>,
    pub attribute_descriptions: Vec<VertexInputAttributeDescription>,
    pub flags: PipelineVertexInputStateCreateFlags,
}

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub color: glm::Vec3,
}

impl Vertex {
    pub fn get_vertex_input_description() -> VertexInputDescription {
        let mut vertex_input_description = VertexInputDescription {
            binding_descriptions: vec![],
            attribute_descriptions: vec![],
            flags: PipelineVertexInputStateCreateFlags::empty(),
        };

        vertex_input_description.binding_descriptions.push(
            VertexInputBindingDescription::default()
                .binding(0)
                .stride(size_of::<Vertex>() as u32)
                .input_rate(VertexInputRate::VERTEX),
        );

        vertex_input_description.attribute_descriptions.push(
            VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(ash::vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, position) as u32),
        );

        vertex_input_description.attribute_descriptions.push(
            VertexInputAttributeDescription::default()
                .binding(0)
                .location(1)
                .format(ash::vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, normal) as u32),
        );

        vertex_input_description.attribute_descriptions.push(
            VertexInputAttributeDescription::default()
                .binding(0)
                .location(2)
                .format(ash::vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, color) as u32),
        );

        vertex_input_description
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Serialize)]
pub struct MeshPushConstants {
    pub data: glm::Vec4,
    pub render_matrix: glm::Mat4,
}

impl MeshPushConstants {}
