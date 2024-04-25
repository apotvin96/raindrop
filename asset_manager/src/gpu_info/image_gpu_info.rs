use ash::vk::Image;
use vk_mem::Allocation;

pub struct ImageGpuInfo {
    pub image: Image,
    pub allocation: Allocation,
}
