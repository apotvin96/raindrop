use std::{mem::size_of, path::Path};

use ash::{
    vk::{
        BufferCreateInfo, BufferUsageFlags, PipelineVertexInputStateCreateFlags,
        VertexInputAttributeDescription, VertexInputBindingDescription, VertexInputRate,
    },
    Device,
};
use gltf::{mesh::Mode, Semantic};
use gpu_allocator::vulkan::{AllocationCreateDesc, AllocationScheme, Allocator};

use memoffset::offset_of;
use rand::Rng;
use serde_derive::Serialize;

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
}

impl Mesh {
    pub fn from_vertices(vertices: Vec<Vertex>) -> Self {
        Mesh {
            vertices,
            vertex_buffer: None,
        }
    }

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

                    return Mesh {
                        vertices,
                        vertex_buffer: None,
                    };
                }
            }
        }

        Mesh {
            vertices: vec![],
            vertex_buffer: None,
        }
    }

    pub fn upload(&mut self, device: &Device, allocator: &mut Allocator) -> Result<(), String> {
        let buffer_create_info = BufferCreateInfo::builder()
            .size((self.vertices.len() * size_of::<Vertex>()) as u64)
            .usage(BufferUsageFlags::VERTEX_BUFFER)
            .build();

        let buffer = match unsafe { device.create_buffer(&buffer_create_info, None) } {
            Ok(buffer) => buffer,
            Err(err) => return Err("Failed to create buffer: ".to_string() + &err.to_string()),
        };

        let allocation_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

        let mut allocation = allocator
            .allocate(&AllocationCreateDesc {
                name: "Vertex Buffer",
                requirements: allocation_requirements,
                location: gpu_allocator::MemoryLocation::CpuToGpu,
                linear: true,
                allocation_scheme: AllocationScheme::GpuAllocatorManaged,
            })
            .unwrap();

        unsafe {
            device
                .bind_buffer_memory(buffer, allocation.memory(), allocation.offset())
                .unwrap();
        }

        let copy_record = match presser::copy_from_slice_to_offset_with_align(
            &self.vertices[..],
            &mut allocation,
            0,
            allocation_requirements.alignment as usize,
        ) {
            Ok(copy_record) => copy_record,
            Err(err) => return Err("Failed to copy vertex data: ".to_string() + &err.to_string()),
        };

        let allocated_buffer = AllocatedBuffer {
            buffer,
            allocation,
            start_offset: copy_record.copy_start_offset,
        };

        self.vertex_buffer = Some(allocated_buffer);

        Ok(())
    }

    pub fn free(&mut self, device: &Device, allocator: &mut Allocator) {
        if let Some(allocated_buffer) = self.vertex_buffer.take() {
            allocator.free(allocated_buffer.allocation).unwrap();

            unsafe {
                device.destroy_buffer(allocated_buffer.buffer, None);
            }
        }
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Serialize)]
pub struct MeshPushConstants {
    pub data: glm::Vec4,
    pub render_matrix: glm::Mat4,
}

impl MeshPushConstants {}
