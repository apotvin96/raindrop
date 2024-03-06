use ash::vk::Buffer;
use vk_mem::Allocation;

pub struct AllocatedBuffer {
    pub buffer: Buffer,
    pub allocation: Allocation,
}
