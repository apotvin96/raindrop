mod mesh_gpu_info;
mod vertex;

use std::{thread, time};

use gltf::mesh::Mode;
use log::warn;
pub use mesh_gpu_info::MeshGpuInfo;
pub use vertex::Vertex;

use crate::asset_info::AssetInfo;

pub struct Mesh {
    pub asset_info: AssetInfo,
    pub gpu_info: Option<MeshGpuInfo>,
    pub vertices: Vec<Vertex>,
    pub vertex_count: u32,
}

impl Mesh {
    pub fn load(&mut self) {
        self.asset_info.status = crate::asset_info::AssetStatus::Loaded;

        let import = gltf::import(&self.asset_info.id);

        let (gltf, buffers, _) = import.unwrap();

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

                    if let Some(indices) = reader.read_indices() {
                        for index in indices.into_u32() {
                            vertices.push(Vertex {
                                position: positions[index as usize],
                                normal: normals[index as usize],
                                color: glm::vec3(1.0, 0.0, 0.0),
                            })
                        }
                    }

                    self.vertex_count = vertices.len() as u32;
                    self.vertices = vertices;

                    return;
                }
            }
        }
    }
}
