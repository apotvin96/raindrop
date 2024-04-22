use ash::vk::Buffer;
use vk_mem::Allocation;

pub struct MeshGpuInfo {
    pub allocation: Allocation,
    pub buffer: Buffer,
}
