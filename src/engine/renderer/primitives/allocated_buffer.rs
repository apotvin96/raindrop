use ash::vk::Buffer;
use gpu_allocator::vulkan::Allocation;

pub struct AllocatedBuffer {
    buffer: Buffer,
    allocation: Allocation
}