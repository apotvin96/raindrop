use ash::vk::Buffer;
use gpu_allocator::vulkan::Allocation;

pub struct AllocatedBuffer {
    pub buffer: Buffer,
    pub allocation: Allocation,
    pub start_offset: usize,
}
