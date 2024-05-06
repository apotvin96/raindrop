use vk_mem::Allocation;

pub struct Image {
    pub image: ash::vk::Image,
    pub allocation: Allocation,
}
