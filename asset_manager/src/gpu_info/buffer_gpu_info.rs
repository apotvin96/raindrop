use ash::vk::Buffer;
use vk_mem::Allocation;

pub struct BufferGpuInfo {
    pub allocation: Allocation,
    pub buffer: Buffer,
}
