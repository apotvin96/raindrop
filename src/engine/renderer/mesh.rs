use std::{mem::size_of, path::Path};

use ash::vk::{
    BufferCreateInfo, BufferUsageFlags, PipelineVertexInputStateCreateFlags,
    VertexInputAttributeDescription, VertexInputBindingDescription, VertexInputRate,
};
use gltf::mesh::Mode;
// use gpu_allocator::vulkan::{AllocationCreateDesc, AllocationScheme, Allocator};

use memoffset::offset_of;
use rand::Rng;
use serde_derive::Serialize;
use vk_mem::{Alloc, AllocationCreateInfo, Allocator};

use super::primitives::AllocatedBuffer;

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
            VertexInputBindingDescription::builder()
                .binding(0)
                .stride(size_of::<Vertex>() as u32)
                .input_rate(VertexInputRate::VERTEX)
                .build(),
        );

        vertex_input_description.attribute_descriptions.push(
            VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(ash::vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, position) as u32)
                .build(),
        );

        vertex_input_description.attribute_descriptions.push(
            VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(ash::vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, normal) as u32)
                .build(),
        );

        vertex_input_description.attribute_descriptions.push(
            VertexInputAttributeDescription::builder()
                .binding(0)
                .location(2)
                .format(ash::vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, color) as u32)
                .build(),
        );

        vertex_input_description
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: Option<AllocatedBuffer>,
    pub vertex_count: u32,
}

impl Mesh {
    pub fn from_path<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let (gltf, buffers, _) = gltf::import(path).unwrap();

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                let mesh = node.mesh().unwrap();

                for primitive in mesh.primitives() {
                    if primitive.mode() != Mode::Triangles {
                        continue;
                    }

                    let mut positions: Vec<glm::Vec3> = vec![];
                    let mut normals: Vec<glm::Vec3> = vec![];

                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                    if let Some(iter) = reader.read_positions() {
                        for position in iter {
                            positions.push(glm::vec3(position[0], position[1], position[2]));
                        }
                    }
                    if let Some(iter) = reader.read_normals() {
                        for normal in iter {
                            normals.push(glm::vec3(normal[0], normal[1], normal[2]));
                        }
                    }

                    let mut vertices: Vec<Vertex> = vec![];

                    let mut rng = rand::thread_rng();
                    if let Some(indices) = reader.read_indices() {
                        for index in indices.into_u32() {
                            vertices.push(Vertex {
                                position: positions[index as usize],
                                normal: normals[index as usize],
                                color: glm::vec3(rng.gen(), rng.gen(), rng.gen()),
                            })
                        }
                    }

                    let vertex_count = vertices.len() as u32;

                    return Mesh {
                        vertices,
                        vertex_buffer: None,
                        vertex_count: vertex_count,
                    };
                }
            }
        }

        Mesh {
            vertices: vec![],
            vertex_buffer: None,
            vertex_count: 0,
        }
    }

    pub fn free(&mut self, allocator: &mut Allocator) {
        if let Some(mut allocated_buffer) = self.vertex_buffer.take() {
            unsafe {
                allocator.destroy_buffer(allocated_buffer.buffer, &mut allocated_buffer.allocation)
            };
        }
    }
}

impl PartialEq for Mesh {
    // TODO: What should I do if I care about comparing the geometry of the mesh while having no buffer?
    // Either both have no vertex buffer or both have the same vertex buffer
    fn eq(&self, other: &Self) -> bool {
        let both_none = self.vertex_buffer.is_none() && other.vertex_buffer.is_none();

        let both_some = self.vertex_buffer.is_some() && other.vertex_buffer.is_some();

        return both_none
            || (both_some
                && self.vertex_buffer.as_ref().unwrap().buffer
                    == other.vertex_buffer.as_ref().unwrap().buffer);
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Serialize)]
pub struct MeshPushConstants {
    pub data: glm::Vec4,
    pub render_matrix: glm::Mat4,
}

impl MeshPushConstants {}
