mod mesh_gpu_info;
mod vertex;

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

        if import.is_err() {
            self.asset_info.status = crate::asset_info::AssetStatus::Invalid;
            return;
        }

        let (gltf, buffers, _) = import.unwrap();

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                let mesh = node.mesh().unwrap();

                for primitive in mesh.primitives() {
                    if primitive.mode() != gltf::mesh::Mode::Triangles {
                        continue;
                    }

                    self.vertices = Mesh::get_triangular_primitive_vertices(&primitive, &buffers);
                    self.vertex_count = self.vertices.len() as u32;

                    return;
                }
            }
        }
    }

    // The mesh has been uploaded to the GPU and we are storing the GPU info for later reference
    pub fn add_gpu_info(&mut self, gpu_info: MeshGpuInfo) {
        self.gpu_info = Some(gpu_info);
    }

    fn get_triangular_primitive_vertices(primitive: &gltf::Primitive, buffers: &[gltf::buffer::Data]) -> Vec<Vertex> {
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

        vertices
    }
}
