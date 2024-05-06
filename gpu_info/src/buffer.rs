use vk_mem::Allocation;

pub struct Buffer {
    pub allocation: Allocation,
    pub buffer: ash::vk::Buffer,
}
