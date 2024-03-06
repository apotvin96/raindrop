use ash::vk::Image;
use vk_mem::Allocation;

pub struct AllocatedImage {
    pub image: Image,
    pub allocation: Allocation,
}
