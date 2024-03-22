use ash::vk::Buffer;
use vk_mem::{Allocation, Allocator};

pub struct AllocatedBuffer {
    pub buffer: Buffer,
    pub allocation: Allocation,
}
